import fs from 'fs';
import path from 'path';
import https from 'https';
import http from 'http';
import { fileURLToPath } from 'url';

// Anchor the audit to the repository root (parent of scripts/) so results are
// identical no matter which working directory the script is invoked from.
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const rootDir = path.resolve(scriptDir, '..');

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
    if (entry.name === 'node_modules' || entry.name === '.git' || entry.name === '.gemini' || entry.name === 'target') continue;
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...getAllMdFiles(fullPath));
    } else if (entry.name.endsWith('.md')) {
      files.push(fullPath);
    }
  }
  return files;
}

// Removes image syntax ![alt](src) before link matching so nested constructs
// like [![alt](img)](href) resolve to the outer href instead of the inner src.
function stripImages(line) {
  return line.replace(/!\[[^\]]*\]\([^)]*\)/g, '');
}

// Extracts heading anchors from a markdown document while ignoring anything
// inside fenced code blocks (``` or ~~~). Without this, commented-out or
// documented heading-like lines inside fences pollute the anchor set and let
// links to phantom anchors pass validation.
function extractHeadings(lines) {
  const anchors = new Set();
  const headingList = [];
  let inFence = false;
  let fenceMarker = null;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const fenceMatch = line.match(/^\s*(```+|~~~+)/);
    if (fenceMatch) {
      if (!inFence) {
        // Opening fence: remember its marker so a longer closing fence is required
        inFence = true;
        fenceMarker = fenceMatch[1][0];
      } else if (fenceMatch[1][0] === fenceMarker) {
        // Closing fence of the same kind
        inFence = false;
        fenceMarker = null;
      }
      continue;
    }
    if (inFence) continue;

    const match = line.match(/^#{1,6}\s+(.+)$/);
    if (match) {
      const headingRaw = match[1];
      const slug = githubSlug(headingRaw);
      anchors.add(slug);
      headingList.push({ line: i + 1, raw: headingRaw, slug });
    }
  }
  return { anchors, headingList };
}

// Validates one external URL with a HEAD request, falling back to GET for
// servers that reject HEAD. Resolves to an object describing the outcome.
// Timeouts and connection errors count as broken; some CDNs block bots with
// unusual status codes, so a small allowlist keeps the audit practical.
function checkExternalUrl(url, timeoutMs = 8000) {
  const BOT_BLOCKED_STATUSES = new Set([403, 406, 429, 999]);
  return new Promise((resolve) => {
    // attempt() follows one request through redirects and a HEAD-to-GET
    // fallback. `target` tracks the current location after redirects.
    const attempt = (target, method, redirectsLeft) => {
      const requester = target.startsWith('https://') ? https : http;
      const req = requester.request(
        target,
        { method, timeout: timeoutMs, headers: { 'User-Agent': 'termodoro-link-checker/1.0' } },
        (res) => {
          if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location && redirectsLeft > 0) {
            // Follow the redirect to its resolved absolute location
            const next = new URL(res.headers.location, target).toString();
            res.resume();
            attempt(next, method, redirectsLeft - 1);
            return;
          }
          // Consume the body so the socket is released back to the pool
          res.resume();
          if (res.statusCode >= 200 && res.statusCode < 400) {
            resolve({ url, ok: true, status: res.statusCode });
          } else if (BOT_BLOCKED_STATUSES.has(res.statusCode)) {
            // Anti-bot responses do not prove a link is dead; accept them
            resolve({ url, ok: true, status: res.statusCode, note: 'bot-blocked, assumed alive' });
          } else if (method === 'HEAD' && res.statusCode >= 400) {
            // Some servers answer 405/501 on HEAD; retry once with GET
            attempt(target, 'GET', 0);
          } else {
            resolve({ url, ok: false, status: res.statusCode });
          }
        }
      );
      req.on('timeout', () => {
        req.destroy(new Error('timeout'));
      });
      req.on('error', () => {
        if (method === 'HEAD') {
          attempt(target, 'GET', 0);
        } else {
          resolve({ url, ok: false, error: 'unreachable or timed out' });
        }
      });
      req.end();
    };
    attempt(url, 'HEAD', 3);
  });
}

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
  const { anchors, headingList } = extractHeadings(lines);
  fileHeadings.set(filePath, { anchors, content, lines, headingList });
}

// Second pass: validate local markdown links [text](target) & gather external URLs
for (const filePath of mdFiles) {
  const { lines } = fileHeadings.get(filePath);
  const relPath = path.relative(rootDir, filePath);

  for (let lineNum = 1; lineNum <= lines.length; lineNum++) {
    // Strip images first so [![alt](img)](href) matches on href
    const line = stripImages(lines[lineNum - 1]);

    // Match [text](link) or <a href="..."> (src= handled by stripImages above)
    const linkRegex = /(?:\[([^\]]*)\]\(([^)]*)\)|href=["']([^"']*)["'])/g;
    let match;

    while ((match = linkRegex.exec(line)) !== null) {
      const linkText = match[1] || 'link';
      // Use nullish coalescing: an empty string target is meaningful (broken),
      // unlike undefined groups, and must not crash the audit.
      const rawTarget = match[2] ?? match[3] ?? '';
      const target = rawTarget.trim();

      // An empty target (for example []() or href="") is always broken
      if (target === '') {
        localErrors.push({
          file: relPath,
          line: lineNum,
          linkText,
          target: '(empty)',
          reason: 'Link has an empty target'
        });
        totalLocalLinksChecked++;
        continue;
      }

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

if (localErrors.length > 0) {
  console.log(`\n❌ Found ${localErrors.length} Broken Local Link(s):\n`);
  for (const err of localErrors) {
    console.log(`  [BROKEN] ${err.file}:${err.line} -> "${err.linkText}" (${err.target})`);
    console.log(`    ↳ ${err.reason}`);
  }
} else {
  console.log(`\n✅ ALL ${totalLocalLinksChecked} local markdown links, images, files, and anchor slugs are 100% VALID!`);
}

// Third pass: actually fetch every unique external URL and report dead ones.
// This is network-dependent; set SKIP_EXTERNAL_LINK_CHECK=1 to skip gracefully
// (for example in offline CI sandboxes), which is reported honestly rather
// than silently claiming the URLs were verified.
const externalErrors = [];
if (process.env.SKIP_EXTERNAL_LINK_CHECK === '1') {
  console.log(`\n⚠️  External URL check SKIPPED (SKIP_EXTERNAL_LINK_CHECK=1); ${externalUrls.size} unique URLs collected but NOT verified.`);
} else if (externalUrls.size > 0) {
  console.log(`\n🌐 Verifying ${externalUrls.size} unique external URL(s)...`);
  const results = await Promise.all(Array.from(externalUrls).map((url) => checkExternalUrl(url)));
  for (const result of results) {
    if (!result.ok) {
      const where = (urlOccurrences.get(result.url) || []).map((o) => `${o.file}:${o.line}`).join(', ');
      externalErrors.push({ url: result.url, status: result.status, error: result.error, where });
    }
  }
  const aliveCount = results.length - externalErrors.length;
  console.log(`   ✓ Alive: ${aliveCount}/${results.length}`);
  if (externalErrors.length > 0) {
    console.log(`\n❌ Found ${externalErrors.length} Broken External URL(s):\n`);
    for (const err of externalErrors) {
      const cause = err.error ? `network error: ${err.error}` : `HTTP ${err.status}`;
      console.log(`  [DEAD] ${err.url} (${cause})`);
      console.log(`    ↳ referenced at: ${err.where}`);
    }
  }
}

console.log(`============================================================\n`);

if (localErrors.length > 0 || externalErrors.length > 0) {
  process.exit(1);
} else {
  process.exit(0);
}
