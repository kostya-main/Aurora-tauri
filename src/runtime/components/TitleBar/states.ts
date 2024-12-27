import { atom } from 'jotai';
import { atomWithReset } from 'jotai/utils'
import { getCurrentWindow } from '@tauri-apps/api/window';

import { getUserData } from '../../../utils';

export const titlebarBackBtn = atom({
    show: false
});

export const titlebarLogout = atom({
    show: false
});

export const titlebarSettingsBtn = atom({
    show: false
});

export const titlebarTitle = atomWithReset({
    show: true,
    text: await getCurrentWindow().title(),
});

export const titlebarUser = atom(
    getUserData().username || ''
);
