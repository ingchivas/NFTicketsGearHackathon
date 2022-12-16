import { Route, Routes } from 'react-router-dom';
import { Home } from './home';
import { Buy } from './buy';

const routes = [
  { path: '/', Page: Home },
  { path: "/buy", Page: Buy }
];

function Routing() {
  const getRoutes = () => routes.map(({ path, Page }) => <Route key={path} path={path} element={<Page />} />);

  return <Routes>{getRoutes()}</Routes>;
}

export { Routing };
