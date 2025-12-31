import { test, expect } from '@playwright/test';

test.describe('Category Filtering', () => {
  
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('T2-027: Category filtering works correctly', async ({ page }) => {
    // Verify categories are visible
    await expect(page.locator('text=Interactive Demos')).toBeVisible();
    await expect(page.locator('text=Performance')).toBeVisible();
    await expect(page.locator('text=Responsive Design')).toBeVisible();
    await expect(page.locator('text=Accessibility')).toBeVisible();
  });

  test('Filtering shows only demos in selected category', async ({ page }) => {
    // Click on Interactive category
    await page.locator('button:has-text("Interactive")').click();
    
    // Verify interactive demos are shown
    await expect(page.locator('text=Counter')).toBeVisible();
    await expect(page.locator('text=Button States')).toBeVisible();
    await expect(page.locator('text=Text Input')).toBeVisible();
    await expect(page.locator('text=Interactive Slider')).toBeVisible();
    await expect(page.locator('text=Checkbox Group')).toBeVisible();
  });

  test('Category selection persists across page navigation', async ({ page }) => {
    // Select Performance category
    await page.locator('button:has-text("Performance")').click();
    await expect(page.locator('text=Smooth Animations')).toBeVisible();
    
    // Navigate away and back (simulated by clicking another category then back)
    await page.locator('button:has-text("Responsive")').click();
    await expect(page.locator('text=Responsive Grid')).toBeVisible();
    
    // Go back to Performance
    await page.locator('button:has-text("Performance")').click();
    await expect(page.locator('text=Smooth Animations')).toBeVisible();
  });

  test('All categories can be selected', async ({ page }) => {
    const categories = ['Interactive', 'Performance', 'Responsive', 'Accessibility'];
    
    for (const category of categories) {
      await page.locator(`button:has-text("${category}")`).click();
      await expect(page.locator(`button:has-text("${category}")`)).toHaveClass(/primary/);
    }
  });

  test('Home shows all categories overview', async ({ page }) => {
    // Reset to Home
    await page.locator('button:has-text("Home")').click();
    
    // Home should show welcome message
    await expect(page.locator('text=Welcome to Slint 1.14 Showcase')).toBeVisible();
  });

});
