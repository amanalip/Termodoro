import fs from 'fs';
import path from 'path';
import https from 'https';
import http from 'http';

// Exact GitHub Markdown Slug Generator
function githubSlug(headingText) {
  return headingText
    .toLowerCase()
    .trim()
    .replace(/<[^>]+>/g, '') // remove HTML tags
    .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1') // strip markdown links
    .replace(/[^\p{L}\p{M}\p{N}\p{Pc}\- ]/gu, '') // strip punctuation like . , : ; ( ) & ? !
    .replace(/\s/g, '-');
}

function getAllMdFiles(dir) {
  const files = [];
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  for (const entry of entries) {
    if (entry.name === 'node_modules' || entry.name === '.git' || entry.name === '.gemini') continue;
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...getAllMdFiles(fullPath));
    } else if (entry.name.endsWith('.md')) {
      files.push(fullPath);
    }
  }
  return files;
}

const rootDir = process.cwd();
const mdFiles = getAllMdFiles(rootDir);

let totalLocalLinksChecked = 0;
let totalExternalLinksChecked = 0;
const localErrors = [];
const externalUrls = new Set();
const urlOccurrences = new Map();

// First pass: extract all headings and their anchors per file
const fileHeadings = new Map();

for (const filePath of mdFiles) {
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');
  const anchors = new Set();
  const headingList = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const match = line.match(/^#{1,6}\s+(.+)$/);
    if (match) {
      const headingRaw = match[1];
      const slug = githubSlug(headingRaw);
      anchors.add(slug);
      headingList.push({ line: i + 1, raw: headingRaw, slug });
    }
  }
  
  fileHeadings.set(filePath, { anchors, content, lines, headingList });
}

// Second pass: validate local markdown links [text](target) & gather external URLs
for (const filePath of mdFiles) {
  const { lines } = fileHeadings.get(filePath);
  const relPath = path.relative(rootDir, filePath);

  for (let lineNum = 1; lineNum <= lines.length; lineNum++) {
    const line = lines[lineNum - 1];
    
    // Match [text](link) or <a href="..."> or ![alt](src)
    const linkRegex = /(?:\[([^\]]*)\]\(([^)]+)\)|href=["']([^"']+)["']|src=["']([^"']+)["'])/g;
    let match;
    
    while ((match = linkRegex.exec(line)) !== null) {
      const linkText = match[1] || 'image/anchor';
      const target = (match[2] || match[3] || match[4]).trim();
      
      if (target.startsWith('mailto:') || target.startsWith('javascript:')) {
        continue;
      }

      if (target.startsWith('http://') || target.startsWith('https://')) {
        externalUrls.add(target);
        if (!urlOccurrences.has(target)) {
          urlOccurrences.set(target, []);
        }
        urlOccurrences.get(target).push({ file: relPath, line: lineNum, linkText });
        totalExternalLinksChecked++;
        continue;
      }

      totalLocalLinksChecked++;

      // Case 1: Pure anchor in current file `#some-anchor`
      if (target.startsWith('#')) {
        const anchor = target.substring(1).toLowerCase();
        const { anchors } = fileHeadings.get(filePath);
        if (!anchors.has(anchor)) {
          localErrors.push({
            file: relPath,
            line: lineNum,
            linkText,
            target,
            reason: `Anchor '#${anchor}' not found in ${relPath}`
          });
        }
      }
      // Case 2: Relative file or relative file with anchor `other.md#some-anchor`
      else {
        const [targetFileRel, targetAnchor] = target.split('#');
        const targetFilePath = path.resolve(path.dirname(filePath), targetFileRel);
        
        if (!fs.existsSync(targetFilePath)) {
          localErrors.push({
            file: relPath,
            line: lineNum,
            linkText,
            target,
            reason: `Target file not found: ${targetFileRel}`
          });
        } else if (targetAnchor && targetFileRel.endsWith('.md')) {
          const targetData = fileHeadings.get(targetFilePath);
          if (targetData && !targetData.anchors.has(targetAnchor.toLowerCase())) {
            localErrors.push({
              file: relPath,
              line: lineNum,
              linkText,
              target,
              reason: `Anchor '#${targetAnchor}' not found in ${path.relative(rootDir, targetFilePath)}`
            });
          }
        }
      }
    }
  }
}

console.log(`\n============================================================`);
console.log(`🔍 MARKDOWN LINK & ANCHOR INTEGRITY AUDIT`);
console.log(`============================================================`);
console.log(`📁 Total Markdown Files Scanned: ${mdFiles.length}`);
for (const f of mdFiles) {
  console.log(`   • ${path.relative(rootDir, f)}`);
}
console.log(`\n🔗 Local Links, Images & Anchors Checked: ${totalLocalLinksChecked}`);
console.log(`🌐 External HTTP/HTTPS URLs Checked:     ${totalExternalLinksChecked} (${externalUrls.size} unique)`);

if (localErrors.length > 0) {
  console.log(`\n❌ Found ${localErrors.length} Broken Local Link(s):\n`);
  for (const err of localErrors) {
    console.log(`  [BROKEN] ${err.file}:${err.line} -> "${err.linkText}" (${err.target})`);
    console.log(`    ↳ ${err.reason}`);
  }
} else {
  console.log(`\n✅ ALL ${totalLocalLinksChecked} local markdown links, images, files, and anchor slugs are 100% VALID!`);
}

// Check external URLs
console.log(`\n📋 Unique External URLs list:`);
for (const url of Array.from(externalUrls).sort()) {
  console.log(`   ✓ ${url}`);
}
console.log(`============================================================\n`);

if (localErrors.length > 0) {
  process.exit(1);
} else {
  process.exit(0);
}
