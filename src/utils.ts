import { LazyStore } from '@tauri-apps/plugin-store';
import { totalMemory } from 'tauri-plugin-system-info-api';

export const store = new LazyStore('config.json', { autoSave: 500 });

export function getUserData() {
    return JSON.parse(sessionStorage.getItem('userData') || '{}');
}

export function setUserData(userData: object) {
    sessionStorage.setItem('userData', JSON.stringify(userData));
}

export function deleteUserData() {
    sessionStorage.removeItem('userData');
}

export function getTotalMemory(): number {
    let memory = 0;
    totalMemory().then((res) => {
        res = memory;
        console.log(res);
    });
    const remainingMemMegabytes = Math.floor(memory / 1024 ** 2) / 2;

    return (
        remainingMemMegabytes -
        (remainingMemMegabytes % 1024) +
        (remainingMemMegabytes % 1024 ? 1024 : 0)
    );
}
