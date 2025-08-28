/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'pitaia-red': {
          50: '#fef2f2',
          100: '#fee2e2', 
          200: '#fecaca',
          500: '#ef4444',
          600: '#dc2626',
          700: '#b91c1c',
          900: '#7f1d1d',
        },
        'pitaia-green': {
          50: '#f0fdf4',
          100: '#dcfce7',
          500: '#22c55e',
          600: '#16a34a',
          700: '#15803d',
          900: '#14532d',
        },
        'pitaia-gray': {
          50: '#f9fafb',
          100: '#f3f4f6',
          600: '#4b5563',
          700: '#374151',
          800: '#1f2937',
          900: '#111827',
        }
      },
      fontFamily: {
        'sans': ['Inter', 'system-ui', 'sans-serif'],
      },
      backgroundImage: {
        'gradient-pitaia': 'linear-gradient(135deg, #ef4444 0%, #22c55e 100%)',
      },
      boxShadow: {
        'pitaia': '0 10px 25px -5px rgba(239, 68, 68, 0.1), 0 4px 6px -2px rgba(34, 197, 94, 0.05)',
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}
