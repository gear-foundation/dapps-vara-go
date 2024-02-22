import { useEffect, useState } from "react";
import { useSetAtom } from "jotai";
// import { ReactComponent as InstagramSVG } from "@/assets/icons/socials-instagram.svg";
// import { ReactComponent as FacebookSVG } from "@/assets/icons/socials-facebook.svg";
// import { ReactComponent as TiktokSVG } from "@/assets/icons/socials-tiktok.svg";
// import { ReactComponent as TwitterXSVG } from "@/assets/icons/socials-twitterx.svg";
// import { ReactComponent as YoutubeSVG } from "@/assets/icons/socials-youtube.svg";
// import mockedPic from "@/assets/icons/mocked-pic.jpg";
import { SEARCH_ATOM } from "@/features/main/atoms";
import { Footer } from "@/components/Footer";
import { mocked } from "./mockedHtml";
import { styles } from "./mockedStyles";

export const SearchedMurkup = () => {
  const setSearchValue = useSetAtom(SEARCH_ATOM);
  const [searchedTemplate, setSearchedTemplate] = useState<string>(mocked);

  useEffect(() => {
    setSearchValue("");
  }, []);

  const cssAppliedContent = (styles: string, body: string) => `
      ${styles}
      ${body}
    `;

  return (
    <main className="app__container">
      <div
        className="app__content"
        dangerouslySetInnerHTML={{
          __html: cssAppliedContent(styles, searchedTemplate),
        }}
      ></div>
      <Footer />
    </main>
  );
};
