import { useSetAtom } from "jotai";
import { SearchInput } from "@/components/SearchInput";
import styles from "./Main.module.scss";
import varaMainLogo from "@/assets/icons/logo-vara-go.png";
import { SEARCH_ATOM } from "../../atoms";
import { useWasmState } from "@/hooks";

export const Main = () => {
  const setVal = useSetAtom(SEARCH_ATOM);
  const { state } = useWasmState("labels", null);
  const tags = state as any[];

  const handleTagClick = (tag: string) => {
    setVal(tag);
  };

  return (
    <div className={styles.container}>
      <img src={varaMainLogo} alt="vara main logo" className={styles.logo} />
      <SearchInput />
      <div className={styles.predefinedTagsWrapper}>
        <h4 className={styles.heading}>Predefined tags...</h4>
        <div className={styles.predefinedTags}>
          {tags?.map((item) => (
            <button className={styles.tag} onClick={() => handleTagClick(item)}>
              {item}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};
