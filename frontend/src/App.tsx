import { useApi, useAccount } from '@gear-js/react-hooks';
import { Routing } from 'pages';
import { Header, ApiLoader } from 'components';
import { withProviders } from 'hocs';
import 'App.scss';

function Component() {
  const { isApiReady } = useApi();
  const { isAccountReady } = useAccount();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <div className="bg-slate-800 text-white">
      <Header isAccountVisible={isAccountReady} />
      <main className='flex flex-col'>
        {isAppReady ? <Routing /> : <ApiLoader />}
      </main>
    </div>
  );
}

export const App = withProviders(Component);
