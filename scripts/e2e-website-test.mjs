/**
 * Termodoro - Playwright End-to-End Automated Testing Suite
 * Tests Desktop, Tablet, and Mobile viewports for all pages and interactive components.
 */

import http from 'http';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { chromium } from 'playwright';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const DOCS_DIR = path.resolve(__dirname, '../docs');
const TEST_SCREENSHOTS_DIR = path.resolve(__dirname, '../docs/assets/tests');

if (!fs.existsSync(TEST_SCREENSHOTS_DIR)) {
  fs.mkdirSync(TEST_SCREENSHOTS_DIR, { recursive: true });
}

const MIME_TYPES = {
  '.html': 'text/html; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.js': 'application/javascript; charset=utf-8',
  '.svg': 'image/svg+xml',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.ico': 'image/x-icon',
  '.json': 'application/json'
};

// 1. Lightweight Local HTTP Static Server
function startServer(port = 8089) {
  return new Promise((resolve) => {
    const server = http.createServer((req, res) => {
      let reqPath = req.url.split('?')[0];
      if (reqPath === '/' || reqPath === '') reqPath = '/index.html';

      const filePath = path.join(DOCS_DIR, reqPath);
      const ext = path.extname(filePath).toLowerCase();
      const contentType = MIME_TYPES[ext] || 'application/octet-stream';

      fs.readFile(filePath, (err, content) => {
        if (err) {
          res.writeHead(404, { 'Content-Type': 'text/plain' });
          res.end(`404 Not Found: ${reqPath}`);
        } else {
          res.writeHead(200, { 'Content-Type': contentType });
          res.end(content);
        }
      });
    });

    server.listen(port, () => {
      resolve(server);
    });
  });
}

// Test Viewport Profiles
const VIEWPORTS = [
  { name: 'Desktop Large (1920x1080)', width: 1920, height: 1080, isMobile: false },
  { name: 'Desktop Standard (1280x800)', width: 1280, height: 800, isMobile: false },
  { name: 'Tablet Portrait (768x1024)', width: 768, height: 1024, isMobile: false },
  { name: 'Mobile Flagship (390x844)', width: 390, height: 844, isMobile: true },
  { name: 'Mobile Medium (375x667)', width: 375, height: 667, isMobile: true },
  { name: 'Mobile Small (320x568)', width: 320, height: 568, isMobile: true }
];

const PAGES = ['index.html', 'features.html', 'faqs.html'];

async function runTestSuite() {
  console.log('🚀 Starting Termodoro Playwright E2E Test Suite...');
  const port = 8089;
  const server = await startServer(port);
  const baseUrl = `http://localhost:${port}`;
  console.log(`🌐 Local HTTP Test Server running at ${baseUrl}\n`);

  const browser = await chromium.launch({
    headless: true,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  let totalTests = 0;
  let passedTests = 0;
  let failedTests = 0;

  function assert(condition, testName) {
    totalTests++;
    if (condition) {
      console.log(`  ✅ PASS: ${testName}`);
      passedTests++;
    } else {
      console.error(`  ❌ FAIL: ${testName}`);
      failedTests++;
    }
  }

  try {
    // ========================================================================
    // Test Suite 1: Responsive Layout & Zero Horizontal Overflow
    // ========================================================================
    console.log('📋 [Suite 1] Responsive Layout & Zero Horizontal Overflow Verification');
    for (const vp of VIEWPORTS) {
      for (const pageName of PAGES) {
        const context = await browser.newContext({
          viewport: { width: vp.width, height: vp.height },
          isMobile: vp.isMobile,
          hasTouch: vp.isMobile
        });
        const page = await context.newPage();
        await page.goto(`${baseUrl}/${pageName}`, { waitUntil: 'domcontentloaded' });

        const overflowCheck = await page.evaluate(() => {
          const docWidth = document.documentElement.scrollWidth;
          const clientWidth = document.documentElement.clientWidth;
          const windowWidth = window.innerWidth;
          const hasOverflow = docWidth > clientWidth;
          return {
            docWidth,
            clientWidth,
            windowWidth,
            hasOverflow
          };
        });

        assert(
          !overflowCheck.hasOverflow,
          `Zero horizontal overflow on ${pageName} (${vp.name}) [scrollWidth: ${overflowCheck.docWidth}px, clientWidth: ${overflowCheck.clientWidth}px]`
        );

        await context.close();
      }
    }

    // ========================================================================
    // Test Suite 2: Mobile Navigation Drawer & Hamburger Menu Interactions
    // ========================================================================
    console.log('\n📋 [Suite 2] Mobile Navigation Drawer & Hamburger Menu Testing');
    {
      const context = await browser.newContext({
        viewport: { width: 390, height: 844 },
        isMobile: true,
        hasTouch: true
      });
      const page = await context.newPage();
      await page.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });

      // 1. Verify mobile hamburger button is visible and desktop menu is hidden
      const toggleVisible = await page.locator('#mobile-menu-toggle').isVisible();
      const navMenuVisible = await page.locator('.nav-menu').isVisible();
      assert(toggleVisible, 'Mobile hamburger button is visible on 390px viewport');
      assert(!navMenuVisible, 'Desktop nav-menu is hidden on 390px viewport');

      // 2. Open drawer
      await page.locator('#mobile-menu-toggle').click();
      await page.waitForTimeout(300);
      const isDrawerOpen = await page.locator('#mobile-nav-drawer').evaluate(el => el.classList.contains('open'));
      const isBackdropOpen = await page.locator('#mobile-nav-backdrop').evaluate(el => el.classList.contains('open'));
      assert(isDrawerOpen, 'Mobile drawer successfully opens upon tapping hamburger button');
      assert(isBackdropOpen, 'Backdrop activates upon opening mobile drawer');

      // Screenshot mobile drawer state
      await page.screenshot({ path: path.join(TEST_SCREENSHOTS_DIR, 'mobile_drawer_open.png') });

      // 3. Test closing drawer with close button
      await page.locator('#mobile-menu-close').click();
      await page.waitForTimeout(300);
      const isDrawerClosed = await page.locator('#mobile-nav-drawer').evaluate(el => !el.classList.contains('open'));
      assert(isDrawerClosed, 'Mobile drawer closes upon clicking close (X) button');

      // 4. Test closing drawer with Escape key
      await page.locator('#mobile-menu-toggle').click();
      await page.waitForTimeout(200);
      await page.keyboard.press('Escape');
      await page.waitForTimeout(300);
      const isEscClosed = await page.locator('#mobile-nav-drawer').evaluate(el => !el.classList.contains('open'));
      assert(isEscClosed, 'Mobile drawer closes upon pressing Escape key');

      await context.close();
    }

    // ========================================================================
    // Test Suite 3: Installation Code Cards & Copy Buttons
    // ========================================================================
    console.log('\n📋 [Suite 3] Installation Code Cards & Copy Button Architecture');
    {
      const context = await browser.newContext({
        viewport: { width: 390, height: 844 },
        permissions: ['clipboard-read', 'clipboard-write']
      });
      const page = await context.newPage();
      await page.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });

      // 1. Verify code-card structure (header bar contains title + copy button)
      const codeCardHeader = await page.locator('#install-panel-linux .code-card-header').isVisible();
      const codeCardTitle = await page.locator('#install-panel-linux .code-card-title').textContent();
      const copyBtnInHeader = await page.locator('#install-panel-linux .code-card-header .copy-btn').isVisible();
      assert(codeCardHeader, '.code-card-header bar exists');
      assert(codeCardTitle.includes('Linux Terminal'), 'Code card displays title in header bar');
      assert(copyBtnInHeader, 'Copy button is anchored inside .code-card-header and not floating');

      // 2. Click copy button
      const copyBtn = page.locator('#install-panel-linux .copy-btn');
      await copyBtn.click();
      await page.waitForTimeout(100);
      const btnText = await copyBtn.textContent();
      assert(btnText.includes('Copied'), 'Copy button changes text feedback to "✓ Copied!"');

      const toastVisible = await page.locator('#toast-notice').evaluate(el => el.classList.contains('show'));
      assert(toastVisible, 'Toast notification is triggered on copy action');

      // 3. Test switching installation tabs
      const macTab = page.locator('.install-tab-btn[data-install="macos"]');
      await macTab.click();
      const macPanelVisible = await page.locator('#install-panel-macos').isVisible();
      const linuxPanelHidden = await page.locator('#install-panel-linux').isVisible();
      assert(macPanelVisible && !linuxPanelHidden, 'Switching install tab to macOS activates macOS code card');

      await context.close();
    }

    // ========================================================================
    // Test Suite 4: 18-Theme Palette Engine & Live Switching
    // ========================================================================
    console.log('\n📋 [Suite 4] 18-Theme Engine & Live Palette Switching');
    {
      const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
      const page = await context.newPage();
      await page.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });

      // 1. Switch via Navbar Dropdown
      const select = page.locator('#nav-theme-select');
      await select.selectOption('rose_pine');
      await page.waitForTimeout(150);
      let themeAttr = await page.locator('html').getAttribute('data-theme');
      assert(themeAttr === 'rose_pine', 'Navbar select changes document data-theme to "rose_pine"');

      // 2. Switch to OLED Phosphor
      await select.selectOption('oled_phosphor');
      await page.waitForTimeout(150);
      themeAttr = await page.locator('html').getAttribute('data-theme');
      const oledBg = await page.evaluate(() => getComputedStyle(document.documentElement).getPropertyValue('--bg-base').trim());
      assert(themeAttr === 'oled_phosphor', 'Theme switched to OLED Phosphor');
      assert(oledBg === '#000000', 'OLED Phosphor applied pure pitch black background token (#000000)');

      // 3. Switch via Theme Card Click in Gallery
      const draculaCard = page.locator('.theme-card[data-theme="dracula"]');
      await draculaCard.click();
      await page.waitForTimeout(150);
      themeAttr = await page.locator('html').getAttribute('data-theme');
      assert(themeAttr === 'dracula', 'Clicking Dracula theme card updates data-theme to "dracula"');

      await context.close();
    }

    // ========================================================================
    // Test Suite 5: Screenshot Showcase Viewer & Keyboard Navigation
    // ========================================================================
    console.log('\n📋 [Suite 5] Interactive Screenshot Showcase & Keyboard Navigation');
    {
      const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
      const page = await context.newPage();
      await page.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });

      // 1. Initial active screenshot
      const initialSrc = await page.locator('#active-showcase-img').getAttribute('src');
      assert(initialSrc.includes('01_timer_view'), 'Showcase initializes with Timer screen [1]');

      // 2. Click Tab 2 (Tasks)
      await page.locator('#tab-ss-2').click();
      await page.waitForTimeout(200);
      const tasksSrc = await page.locator('#active-showcase-img').getAttribute('src');
      const tasksCap = await page.locator('#active-showcase-caption').textContent();
      assert(tasksSrc.includes('02_task_manager') || tasksSrc.includes('02_tasks_view'), 'Showcase switched to Tasks view [2]');
      assert(tasksCap.includes('Task Management'), 'Showcase updated caption for Tasks view');

      // 3. Press Key '3' (Stats)
      await page.keyboard.press('3');
      await page.waitForTimeout(200);
      const statsSrc = await page.locator('#active-showcase-img').getAttribute('src');
      assert(statsSrc.includes('03_stats_view'), 'Keyboard shortcut [3] switched showcase to Stats view');

      await context.close();
    }

    // ========================================================================
    // Test Suite 6: Keybindings Live Search Filtering
    // ========================================================================
    console.log('\n📋 [Suite 6] Keybindings Live Search Filtering');
    {
      const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
      const page = await context.newPage();
      await page.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });

      const searchInput = page.locator('#keybinding-search');
      await searchInput.fill('reset');
      await page.waitForTimeout(100);

      const visibleRows = await page.evaluate(() => {
        const rows = Array.from(document.querySelectorAll('.keybindings-table tbody tr'));
        return rows.filter(r => r.style.display !== 'none').length;
      });

      assert(visibleRows >= 1, `Searching "reset" filters keybindings table (showing ${visibleRows} matching rows)`);
      await context.close();
    }

    // ========================================================================
    // Test Suite 7: FAQ Hub Search & Category Filtering (faqs.html)
    // ========================================================================
    console.log('\n📋 [Suite 7] FAQ Hub Search & Category Filtering (faqs.html)');
    {
      const context = await browser.newContext({ viewport: { width: 390, height: 844 }, isMobile: true });
      const page = await context.newPage();
      await page.goto(`${baseUrl}/faqs.html`, { waitUntil: 'networkidle' });

      // 1. Pill filter click
      const timerPill = page.locator('.faq-pill-btn[data-category="timer"]');
      if (await timerPill.isVisible()) {
        await timerPill.click();
        await page.waitForTimeout(100);
        const timerGroupVisible = await page.locator('.faq-group[data-group="timer"]').isVisible();
        const archGroupHidden = await page.locator('.faq-group[data-group="arch"]').isVisible();
        assert(timerGroupVisible && !archGroupHidden, 'FAQ Category pill filtering isolates selected category');
      }

      // 2. Search filtering
      const faqSearch = page.locator('#faq-search-input');
      await faqSearch.fill('tmux');
      await page.waitForTimeout(150);
      const visibleFaqs = await page.evaluate(() => {
        const items = Array.from(document.querySelectorAll('.faq-item'));
        return items.filter(i => i.style.display !== 'none').length;
      });
      assert(visibleFaqs >= 1, `FAQ query "tmux" returns ${visibleFaqs} matching questions`);

      await context.close();
    }

    // ========================================================================
    // Visual Verification Screenshots Capture
    // ========================================================================
    console.log('\n📸 Capturing Visual Verification Artifacts for Desktop & Mobile...');
    {
      // Desktop Screenshot
      const dContext = await browser.newContext({ viewport: { width: 1280, height: 800 } });
      const dPage = await dContext.newPage();
      await dPage.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });
      await dPage.screenshot({ path: path.join(TEST_SCREENSHOTS_DIR, 'desktop_overview.png'), fullPage: false });
      await dContext.close();

      // Mobile Screenshot
      const mContext = await browser.newContext({ viewport: { width: 390, height: 844 }, isMobile: true });
      const mPage = await mContext.newPage();
      await mPage.goto(`${baseUrl}/index.html`, { waitUntil: 'networkidle' });
      await mPage.screenshot({ path: path.join(TEST_SCREENSHOTS_DIR, 'mobile_overview.png'), fullPage: false });
      await mContext.close();
    }

  } finally {
    await browser.close();
    server.close();
  }

  console.log('\n' + '='.repeat(60));
  console.log(`📊 TEST SUITE SUMMARY: ${passedTests}/${totalTests} PASSED (${failedTests} FAILED)`);
  console.log('='.repeat(60) + '\n');

  if (failedTests > 0) {
    process.exit(1);
  }
}

runTestSuite().catch(err => {
  console.error('Test Suite Exception:', err);
  process.exit(1);
});
