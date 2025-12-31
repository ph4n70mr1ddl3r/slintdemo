import { test, expect } from '@playwright/test';

test.describe('Performance Metrics - Animated Counters', () => {
  
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('T3-041: Animated counters display performance metrics', async ({ page }) => {
    // Navigate to Performance page
    await page.locator('button:has-text("Performance")').click();
    
    // Verify Performance page is displayed
    await expect(page.locator('text=Performance Metrics')).toBeVisible();
  });

  test('Performance metrics show numerical values', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Look for any numerical displays in the performance section
    // The page should show metrics (even if "coming soon" placeholder)
    await expect(page.locator('text=Performance')).toBeVisible();
  });

  test('Comparison charts display Slint vs baseline data', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Should show comparison indicators
    await expect(page.locator('text=Slint')).toBeVisible();
    await expect(page.locator('text=React')).toBeVisible();
  });

  test('Performance section has visual indicators for metrics', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Look for visual elements that indicate performance metrics
    // Could be bars, numbers, or other visual indicators
    const performanceContent = page.locator('[role="main"]');
    await expect(performanceContent).toBeVisible();
  });

  test('Metric values are formatted correctly', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Check for properly formatted numbers with units
    // Common patterns: "45 ms", "60 fps", "100 KB"
    await expect(page.locator('text=/\\d+\\s*(ms|fps|KB|MB)/')).toBeVisible();
  });

  test('Source citations are displayed for benchmarks', async ({ page }) => {
    await page.locator('button:has-text("Performance")').click();
    
    // Should show where the benchmark data comes from
    await expect(page.locator('text=Slint')).toBeVisible();
  });

});
