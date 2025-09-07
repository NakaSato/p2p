import { render, screen } from '@testing-library/react'
import { describe, it, expect } from 'vitest'
import { BrowserRouter } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import App from '../App'

const createTestQueryClient = () => new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
    },
  },
})

const renderWithProviders = (ui: React.ReactElement) => {
  const queryClient = createTestQueryClient()
  return render(
    <QueryClientProvider client={queryClient}>
      <BrowserRouter>
        {ui}
      </BrowserRouter>
    </QueryClientProvider>
  )
}

describe('App', () => {
  it('renders without crashing', () => {
    renderWithProviders(<App />)
    // Check for the main dashboard heading (not navigation links)
    expect(screen.getByRole('heading', { name: 'Dashboard' })).toBeInTheDocument()
  })

  it('displays navigation menu', () => {
    renderWithProviders(<App />)
    // Check for navigation items by using getAllByRole to handle duplicates
    const tradingLinks = screen.getAllByRole('link', { name: /trading/i })
    expect(tradingLinks.length).toBeGreaterThan(0)
    
    const profileLinks = screen.getAllByRole('link', { name: /profile/i })
    expect(profileLinks.length).toBeGreaterThan(0)
    
    const settingsLinks = screen.getAllByRole('link', { name: /settings/i })
    expect(settingsLinks.length).toBeGreaterThan(0)
    
    // Check for the logo/title - use getAllByText and check first one
    const titles = screen.getAllByText('P2P Energy')
    expect(titles.length).toBeGreaterThan(0)
  })

  it('displays dashboard content', () => {
    renderWithProviders(<App />)
    // Check for some actual dashboard content that exists
    expect(screen.getByText('Energy Production')).toBeInTheDocument()
    expect(screen.getByText('Recent Trades')).toBeInTheDocument()
    expect(screen.getByText('Grid Overview')).toBeInTheDocument()
  })
})
