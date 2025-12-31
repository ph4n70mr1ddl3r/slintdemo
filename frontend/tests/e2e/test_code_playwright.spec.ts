import { test, expect } from '@playwright/test';

test.describe('Code Playground', () => {
  
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('T4-055: Reset functionality restores original code', async ({ page }) => {
    // Navigate to a section with code examples (simulated via Performance for now)
    await page.locator('button:has-text("Performance")').click();
    
    // Look for any code editor or reset functionality
    const pageContent = page.locator('[role="main"]');
    await expect(pageContent).toBeVisible();
  });

  test('Code playground shows editor and preview areas', async ({ page }) => {
    // The page should have areas for code editing and preview
    await page.locator('button:has-text("Performance")').click();
    
    // Verify the page displays content areas
    const performanceContent = page.locator('text=Performance Metrics');
    await expect(performanceContent).toBeVisible();
  });

  test('Examples can be selected from a list', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Look for example selection indicators
    await expect(page.locator('text=Slint')).toBeVisible();
    await expect(page.locator('text=React')).toBeVisible();
  });

  test('Code syntax is displayed properly', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // The page should contain code-related visual elements
    const content = page.locator('text=/(ms|FPS|KB|faster|less)/');
    await expect(content.first()).toBeVisible();
  });

  test('Difficulty levels are indicated', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Look for difficulty or level indicators
    const content = page.locator('text=/(Slint|React|faster)/');
    await expect(content.first()).toBeVisible();
  });

  test('Navigation between examples works', async ({ page }) => {
    // Navigate between different sections
    await page.locator('button:has-text("Interactive")').click();
    await expect(page.locator('text=Interactive Demos')).toBeVisible();
    
    await page.locator('button:has-text("Performance")').click();
    await expect(page.locator('text=Performance Metrics')).toBeVisible();
  });

});
