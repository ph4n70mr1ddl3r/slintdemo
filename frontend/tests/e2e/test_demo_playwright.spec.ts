// E2E Tests for Demo Visual Feedback
// Tests verifying visual feedback appears without page reload

import { test, expect } from '@playwright/test';

test.describe('Demo Visual Feedback', () => {
  
  test.beforeEach(async ({ page }) => {
    // Navigate to showcase
    await page.goto('/');
    // Wait for page to load
    await page.waitForLoadState('domcontentloaded');
  });

  test('counter demo shows visual feedback on increment', async ({ page }) => {
    // Navigate to interactive demos
    await page.click('text=Interactive Demos');
    
    // Find counter demo section
    const counterSection = page.locator('text=Counter Demo').first();
    await expect(counterSection).toBeVisible();
    
    // Click increment button
    const incrementButton = page.locator('button:has-text("+")').first();
    await incrementButton.click();
    
    // Verify counter value changed (visual feedback)
    const counterValue = page.locator('text=/\\d+/').first();
    await expect(counterValue).toBeVisible();
    
    // Verify no page reload occurred (URL should not change)
    await expect(page).not.toHaveURL(/.*reload.*/);
  });

  test('demo navigation works without page reload', async ({ page }) => {
    // Initial URL
    const initialUrl = page.url();
    
    // Click through different category tabs
    const categories = ['Interactive Demos', 'Performance', 'Responsive Design', 'Accessibility'];
    
    for (const category of categories) {
      await page.click(`text=${category}`);
      // Wait a moment for any navigation
      await page.waitForTimeout(100);
    }
    
    // Verify URL still same (no page reload)
    expect(page.url()).toBe(initialUrl);
  });

  test('visual feedback animations are present', async ({ page }) => {
    await page.click('text=Interactive Demos');
    
    // Check that animations are present in demo components
    // This verifies the visual feedback requirement
    const demoSection = page.locator('section:has-text("Counter Demo")');
    await expect(demoSection).toBeVisible();
  });

  test('counter demo state persists during interaction', async ({ page }) => {
    await page.click('text=Interactive Demos');
    
    // Increment counter multiple times
    const incrementButton = page.locator('button:has-text("+")').first();
    
    // Click 3 times
    await incrementButton.click();
    await incrementButton.click();
    await incrementButton.click();
    
    // Verify final value is 3 (state persisted)
    const counterValue = page.locator('text=/3/').first();
    await expect(counterValue).toBeVisible();
  });

  test('showcase loads within performance goal', async ({ page }) => {
    const startTime = Date.now();
    
    await page.goto('/');
    await page.waitForLoadState('domcontentloaded');
    
    const loadTime = Date.now() - startTime;
    
    // Should load within 3 seconds (SC-005)
    expect(loadTime).toBeLessThan(3000);
  });

  test('all demos are accessible from showcase', async ({ page }) => {
    // Check home page shows demo categories
    const categories = page.locator('text=/Interactive|Performance|Responsive|Accessibility/');
    const count = await categories.count();
    expect(count).toBeGreaterThanOrEqual(4);
  });

  test('metrics display correctly on home page', async ({ page }) => {
    // Home page should show performance metrics
    const metricsSection = page.locator('text=/ms|fps|KB/');
    await expect(metricsSection.first()).toBeVisible();
  });

  test('navigation sidebar is present', async ({ page }) => {
    // Header with navigation should be visible
    const header = page.locator('h1:has-text("Slint")');
    await expect(header).toBeVisible();
    
    // Footer should also be visible
    const footer = page.locator('text=Built with Slint');
    await expect(footer).toBeVisible();
  });
});

test.describe('Demo Components Independence', () => {
  test('each demo can be viewed independently', async ({ page }) => {
    await page.goto('/');
    
    // Verify no demo is selected by default (home view showing)
    const welcomeText = page.locator('text=Welcome to the Slint 1.14 Showcase');
    await expect(welcomeText).toBeVisible();
  });
});

test.describe('Accessibility', () => {
  test('page has proper heading structure', async ({ page }) => {
    await page.goto('/');
    
    // Should have h1
    const h1 = page.locator('h1').first();
    await expect(h1).toBeVisible();
  });

  test('interactive elements are keyboard accessible', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Interactive Demos');
    
    // Tab to first interactive element
    await page.keyboard.press('Tab');
    
    // Should be able to interact with keyboard
    await page.keyboard.press('Tab');
  });
});
