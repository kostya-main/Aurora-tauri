import Modal from './Modal';
import TitleBar from './TitleBar';
import type { PropsWithChildren } from 'react';

export default function Layout({children}: PropsWithChildren<{}>) {
    return (
        <>
            <TitleBar />
            <main>
                {children}
            </main>
            <Modal />
        </>
    );
}
