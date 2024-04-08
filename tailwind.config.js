module.exports = {
  mode: 'jit',
  content: {
    files: ['src/**/*.rs', 'index.html'],
  },
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        neutralBg: 'var(--neutralBg)',
        onNeutralBg: 'var(--onNeutralBg)',
        boardPrimaryBg: 'var(--boardPrimaryBg)',
        chipPrimaryBg: 'var(--chipPrimaryBg)',
        chipSecondaryBg: 'var(--chipSecondaryBg)',
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
};
