import { useApi, useAccount } from "@gear-js/react-hooks";


function Account() {
  const { isApiReady } = useApi();
  const { isAccountReady, account, accounts } = useAccount();

  const isAppReady = isApiReady && isAccountReady;

  return (
    <div>
      {isAppReady ? "true" : "loading"}
      {account ? "cuenta" : "no cuenta"}
    </div>
  );
}

export default Account;
