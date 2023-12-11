export interface ColorTheme {
    name: string;
    foreground: {
        primary: string;   // --fg
        secondary: string; // --fg2
    };
    background: {
        primary: string;   // --bg
        secondary: string; // --bg2
        error: string;     // --bg-err
    };
}

export interface StyleTheme {
    name: string;
    fontFamily: {
        text: string; // --text
        code: string; // --code
    };
    fontSizes: string[]    // 9 values, --f-4 through --f4
    paddingSizes: string[] // 5 values, --xs through --xl
}

const dark: ColorTheme = {
    name: 'Dark',
    foreground: {
        primary: '#ffffff',
        secondary: '#888888'
    },
    background: {
        primary: '#000000',
        secondary: '#222222',
        // TODO
        error: 'red'
    }
};

const light: ColorTheme = {
    name: 'Light',
    foreground: {
        primary: '#000000',
        secondary: '#222222'
    },
    background: {
        primary: '#ffffff',
        secondary: '#dddddd',
        // TODO
        error: 'red'
    }
};

const balanced: StyleTheme = {
    name: 'Balanced',
    fontFamily: {
        text: 'IBM Plex Sans',
        code: 'IBM Plex Mono'
    },
    fontSizes: [
        '0.25rem',
        '0.35rem',
        '0.50rem',
        '0.71rem',
        '1.00rem',
        '1.41rem',
        '2.00rem',
        '2.83rem',
        '4.00rem'
    ],
    paddingSizes: [
        '0.13rem',
        '0.25rem',
        '0.50rem',
        '1.00rem',
        '2.00rem'
    ]
};

const spaced: StyleTheme = {
    name: 'Spaced',
    fontFamily: {
        text: 'IBM Plex Sans',
        code: 'IBM Plex Mono'
    },
    fontSizes: [
        '0.18rem',
        '0.25rem',
        '0.35rem',
        '0.50rem',
        '0.71rem',
        '1.41rem',
        '2.00rem',
        '2.83rem',
        '4.00rem'
    ],
    paddingSizes: [
        '0.13rem',
        '0.25rem',
        '0.50rem',
        '1.00rem',
        '2.00rem'
    ]
};

export const setColorTheme = (theme: ColorTheme) => {
    const style = document.documentElement.style;
    
    style.setProperty('--fg',  theme.foreground.primary);
    style.setProperty('--fg2', theme.foreground.secondary);

    style.setProperty('--bg',     theme.background.primary);
    style.setProperty('--bg2',    theme.background.secondary);
    style.setProperty('--bg-err', theme.background.error);
};

export const setStyleTheme = (theme: StyleTheme) => {
    const style = document.documentElement.style;

    style.setProperty('--text', theme.fontFamily.text);
    style.setProperty('--code', theme.fontFamily.code);

    // Too bad there's not a better way to do this, loops haven't been invented yet :/
    style.setProperty('--f-4', theme.fontSizes[0]);
    style.setProperty('--f-3', theme.fontSizes[1]);
    style.setProperty('--f-2', theme.fontSizes[2]);
    style.setProperty('--f-1', theme.fontSizes[3]);
    style.setProperty('--f0',  theme.fontSizes[4]);
    style.setProperty('--f1',  theme.fontSizes[5]);
    style.setProperty('--f2',  theme.fontSizes[6]);
    style.setProperty('--f3',  theme.fontSizes[7]);
    style.setProperty('--f4',  theme.fontSizes[8]);

    style.setProperty('--xs', theme.paddingSizes[0]);
    style.setProperty('--sm', theme.paddingSizes[1]);
    style.setProperty('--md', theme.paddingSizes[2]);
    style.setProperty('--lg', theme.paddingSizes[3]);
    style.setProperty('--xl', theme.paddingSizes[4]);
}

export const colorThemes = [dark, light];
export const styleThemes = [balanced, spaced];