import { Footer } from "@/components/Footer";
import styles from "./SearchResults.module.scss";
import { Link, useNavigate } from "react-router-dom";
import { useAtomValue } from "jotai";
import { SEARCH_ATOM } from "@/features/main/atoms";
import { useWasmState } from "@/hooks";
import { useEffect, useState } from "react";

const results: any[] = [
  {
    description:
      "Hey! I'm Kara! We teach you how to make sustainable money choices that benefit you and our world.",
    domain: "vara.kara-blog",
  },
  {
    description: `My Trip to Paris! Last month I took a trip to the beautiful city of Paris. It was my first time visiting and I had an amazing time exploring the city.
    Some of the highlights included:`,
    domain: "vara.travel-blog",
  },
  {
    description:
      "Some of the features that make Wolfram Alpha stand out from other search engines include ",
    domain: "vara.some-another-domain",
  },
  {
    description:
      "A search engine is a web based tool that is used by people to locate information on the internet. Some of the most popular examples of search engines are Google",
    domain: "vara.some-another-domain",
  },
  {
    description:
      "Below are some tried and true web search techniques that work with virtually any search",
    domain: "vara.some-another-domain",
  },
];

export const SearchResults = () => {
  const navigate = useNavigate();
  const label = useAtomValue(SEARCH_ATOM);
  const [res, setRes] = useState<any[]>([]);

  useEffect(() => {
    console.log(label);
    if (label === "Apple") {
      setRes(results);
    } else {
      setRes([]);
    }
  }, [label, setRes]);

  // useEffect(() => {
  //   if (label === undefined) {
  //     navigate("/");
  //   }
  // }, [label]);

  return (
    <>
      <div className={styles.container}>
        <ul className={styles.searchResults}>
          {res?.length
            ? res?.map((item) => (
                <li className={styles.result}>
                  <Link
                    className={styles.resultName}
                    to={`/res/${item.domain}`}
                  >
                    {item.description}
                  </Link>
                  <span className={styles.resultUrl}>{item.domain}</span>
                </li>
              ))
            : "No results found"}
        </ul>
      </div>
      <Footer />
    </>
  );
};
