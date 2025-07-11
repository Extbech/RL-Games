import { createTheme } from '@mui/material';
import { responsiveFontSizes } from '@mui/material/styles';

const lightPalette = {
  background: {
    default: '#ffffff',
    paper: '#f5f5f5',
  },
  primary: {
    main: '#1976d2',
  },
  secondary: {
    main: '#dc004e',
  },
  text: {
    primary: '#000000',
    secondary: '#666666',
  },
};

const darkPalette = {
  background: {
    default: '#121212',
    paper: '#1e1e1e',
  },
  primary: {
    main: '#bb86fc',
  },
  secondary: {
    main: '#03dac6',
  },
  text: {
    primary: '#ffffff',
    secondary: '#bbbbbb',
  },
};

export const theme = createTheme({
  typography: {
    fontFamily: [
      'Quicksand',
      'cursive',
    ].join(','),
  },
  palette: {
    mode: 'light',
    ...lightPalette,
  },
  components: {
    MuiCssBaseline: {
      styleOverrides: {
        body: {
          backgroundColor: lightPalette.background.default,
        },
      },
    },
  },
  breakpoints: {
    values: {
      xs: 0,
      sm: 600,
      md: 960,
      lg: 1280,
      xl: 1920,
    },
  },
});

export type Mode = 'light' | 'dark';
// Function to switch between light and dark mode
export const getTheme = (mode: Mode) => {
  return createTheme({
    ...theme,
    palette: {
      mode,
      ...(mode === 'dark' ? darkPalette : lightPalette),
    },
    components: {
      MuiCssBaseline: {
        styleOverrides: {
          body: {
            backgroundColor: mode === 'dark' ? darkPalette.background.default : lightPalette.background.default,
          },
        },
      },
    },
  });
};

export const lightTheme = getTheme('light');
export const darkTheme = getTheme('dark');

// Make fonts responsive
export const responsiveTheme = responsiveFontSizes(theme);