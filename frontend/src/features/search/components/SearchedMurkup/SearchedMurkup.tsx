import { useEffect, useState } from "react";
import { useSetAtom } from "jotai";
import styles from "./SearchedMurkup.module.scss";
// import { ReactComponent as InstagramSVG } from "@/assets/icons/socials-instagram.svg";
// import { ReactComponent as FacebookSVG } from "@/assets/icons/socials-facebook.svg";
// import { ReactComponent as TiktokSVG } from "@/assets/icons/socials-tiktok.svg";
// import { ReactComponent as TwitterXSVG } from "@/assets/icons/socials-twitterx.svg";
// import { ReactComponent as YoutubeSVG } from "@/assets/icons/socials-youtube.svg";
// import mockedPic from "@/assets/icons/mocked-pic.jpg";
import { SEARCH_ATOM } from "@/features/main/atoms";
import { Footer } from "@/components/Footer";
import { mocked, mocked1, mocked2 } from "./mockedHtml";
import { styles as st, styles1, styles2 } from "./mockedStyles";
import { useParams } from "react-router-dom";

export const SearchedMurkup = () => {
  const { id } = useParams();
  const setSearchValue = useSetAtom(SEARCH_ATOM);
  // const [searchedTemplate, setSearchedTemplate] = useState<string>(mocked);

  useEffect(() => {
    setSearchValue("");
  }, []);

  const defineStyles = (id: string | undefined) => {
    if (id === "vara.kara-blog") {
      return st;
    }

    if (id === "vara.travel-blog") {
      return styles1;
    }

    return styles2;
  };

  const defineHtml = (id: string | undefined) => {
    if (id === "vara.kara-blog") {
      return mocked;
    }

    if (id === "vara.travel-blog") {
      return mocked1;
    }

    return mocked2;
  };

  const cssAppliedContent = (styles: string, body: string) => `
      ${styles}
      ${body}
    `;

  return (
    <main className={styles.container}>
      <div
        className={styles.content}
        dangerouslySetInnerHTML={{
          __html: cssAppliedContent(defineStyles(id), defineHtml(id)),
        }}
      ></div>
      <Footer />
    </main>
  );
};
