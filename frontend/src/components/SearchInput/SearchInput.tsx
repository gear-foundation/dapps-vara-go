import { useEffect, useCallback } from "react";
import { useAtom } from "jotai";
import styles from "./SearchInput.module.scss";
import { useNavigate } from "react-router-dom";
import { SEARCH_ATOM } from "@/features/main/atoms";

type Props = {};
export const SearchInput = ({}: Props) => {
  const navigate = useNavigate();
  const [val, setVal] = useAtom(SEARCH_ATOM);

  const handleChangeValue = (e: React.ChangeEvent<HTMLInputElement>) => {
    setVal(e.target.value);
  };

  const handleSetValue = (question: string) => {
    if (question) {
      navigate(`/search/${question}`);
    }
  };

  const handleSearchButtonClick = () => {
    handleSetValue(val);
  };

  const handleKeyClick = useCallback(
    (event: React.KeyboardEvent<HTMLDivElement> | KeyboardEvent) => {
      if (event.key === "Enter") {
        event.preventDefault();

        handleSetValue(val);
      }
    },
    [handleSetValue]
  );

  useEffect(() => {
    document.documentElement.addEventListener("keydown", handleKeyClick);

    return () => {
      document.documentElement.removeEventListener("keydown", handleKeyClick);
    };
  }, [handleKeyClick]);

  return (
    <div className={styles.searchWrapper}>
      <input
        type="text"
        placeholder="Search something..."
        className={styles.search}
        value={val}
        maxLength={160}
        onChange={handleChangeValue}
      />
      <button className={styles.searchButton} onClick={handleSearchButtonClick}>
        Search
      </button>
    </div>
  );
};
