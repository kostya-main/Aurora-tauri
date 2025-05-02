import { Route } from 'wouter-preact';
import { Provider } from 'jotai';

import Layout from './runtime/components/Layout';
import Login from './runtime/scenes/Login';
import ServerPanel from './runtime/scenes/ServerPanel';
import ServersList from './runtime/scenes/ServersList';
import Settings from './runtime/scenes/Settings';

export default function App() {
    return (
        <Provider>
            <Layout>
                <Route path="/" component={Login} />
                <Route path="/ServersList" component={ServersList} />
                <Route path="/ServerPanel" component={ServerPanel} />
                <Route path="/Settings" component={Settings} />
            </Layout>
        </Provider>
    );
}
