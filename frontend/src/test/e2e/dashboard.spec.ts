import { test, expect } from '@playwright/test';

test.describe('Dashboard Page', () => {
  test('should load dashboard and display energy stats', async ({ page }) => {
    await page.goto('/');
    
    // Check if the page title is correct
    await expect(page).toHaveTitle(/P2P Energy Trading/);
    
    // Check if dashboard heading is visible
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    
    // Check if energy stats cards are present
    await expect(page.getByText('Energy Sold')).toBeVisible();
    await expect(page.getByText('Energy Bought')).toBeVisible();
    await expect(page.getByText('Earnings')).toBeVisible();
    await expect(page.getByText('Active Trades')).toBeVisible();
  });

  test('should navigate between pages', async ({ page }) => {
    await page.goto('/');
    
    // Navigate to Trading page
    await page.getByRole('link', { name: 'Trading' }).click();
    await expect(page.getByRole('heading', { name: 'Trading' })).toBeVisible();
    
    // Navigate to Profile page
    await page.getByRole('link', { name: 'Profile' }).click();
    await expect(page.getByRole('heading', { name: 'Profile' })).toBeVisible();
    
    // Navigate to Settings page
    await page.getByRole('link', { name: 'Settings' }).click();
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();
  });
});
