import fs from 'fs';
import path from 'path';

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

let totalLinksChecked = 0;
const errors = [];

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

// Second pass: validate every markdown link [text](target)
for (const filePath of mdFiles) {
  const { lines } = fileHeadings.get(filePath);
  const relPath = path.relative(rootDir, filePath);

  for (let lineNum = 1; lineNum <= lines.length; lineNum++) {
    const line = lines[lineNum - 1];
    
    // Match [text](link)
    const linkRegex = /\[([^\]]*)\]\(([^)]+)\)/g;
    let match;
    
    while ((match = linkRegex.exec(line)) !== null) {
      const [fullMatch, linkText, target] = match;
      
      // Ignore external http/https/mailto/file links
      if (target.startsWith('http://') || target.startsWith('https://') || target.startsWith('mailto:') || target.startsWith('file://')) {
        continue;
      }

      totalLinksChecked++;

      // Case 1: Pure anchor in current file `#some-anchor`
      if (target.startsWith('#')) {
        const anchor = target.substring(1).toLowerCase();
        const { anchors } = fileHeadings.get(filePath);
        if (!anchors.has(anchor)) {
          errors.push({
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
          errors.push({
            file: relPath,
            line: lineNum,
            linkText,
            target,
            reason: `Target file not found: ${targetFileRel}`
          });
        } else if (targetAnchor && targetFileRel.endsWith('.md')) {
          const targetData = fileHeadings.get(targetFilePath);
          if (targetData && !targetData.anchors.has(targetAnchor.toLowerCase())) {
            errors.push({
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

console.log(`\n🔗 Markdown Link Validation Results:`);
console.log(`Total Markdown Files Scanned: ${mdFiles.length}`);
console.log(`Total Local Links & Anchors Checked: ${totalLinksChecked}`);

if (errors.length > 0) {
  console.log(`\n❌ Found ${errors.length} Broken Link(s):\n`);
  for (const err of errors) {
    console.log(`  [BROKEN] ${err.file}:${err.line} -> "${err.linkText}" (${err.target})`);
    console.log(`    ↳ ${err.reason}`);
  }
} else {
  console.log(`\n✅ All ${totalLinksChecked} local markdown links and anchors are 100% valid!\n`);
}
