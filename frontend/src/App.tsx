import { Routes, Route } from "react-router-dom";
import styles from "./App.module.scss";
import { withProviders } from "./hocs";
import "@gear-js/vara-ui/dist/style.css";
import { Main } from "./features/main/components/Main";
import { SearchResults } from "./features/search/components/SearchResults";
import { Header } from "./components/Header";
import { SearchedMurkup } from "./features/search/components/SearchedMurkup";

const Component = () => {
  return (
    <div className={styles.app}>
      <Routes>
        <Route path="/" element={<Main />} />
        <Route
          path="/*"
          element={
            <>
              <Header />
              <div className={styles.searchBox}>
                <Routes>
                  <Route path={`/search/:id`} element={<SearchResults />} />
                  <Route path={`/res/:id`} element={<SearchedMurkup />} />
                </Routes>
              </div>
            </>
          }
        />
      </Routes>
    </div>
  );
};

export const App = withProviders(Component);
