import { Footer } from "@/components/Footer";
import styles from "./SearchResults.module.scss";
import { Link } from "react-router-dom";

const results = [
  {
    name: "Hey! I'm Kara! We teach you how to make sustainable money choices that benefit you and our world.",
    url: "vara.kara-blog",
  },
  {
    name: `My Trip to Paris! Last month I took a trip to the beautiful city of Paris. It was my first time visiting and I had an amazing time exploring the city.
    Some of the highlights included:`,
    url: "vara.travel-blog",
  },
  {
    name: "Some of the features that make Wolfram Alpha stand out from other search engines include ",
    url: "vara.some-another-domain",
  },
  {
    name: "A search engine is a web based tool that is used by people to locate information on the internet. Some of the most popular examples of search engines are Google",
    url: "vara.some-another-domain",
  },
  {
    name: "Below are some tried and true web search techniques that work with virtually any search",
    url: "vara.some-another-domain",
  },
];

export const SearchResults = () => {
  return (
    <>
      <div className={styles.container}>
        <ul className={styles.searchResults}>
          {results.map((item) => (
            <li className={styles.result}>
              <Link className={styles.resultName} to={`/res/${item.url}`}>
                {item.name}
              </Link>
              <span className={styles.resultUrl}>{item.url}</span>
            </li>
          ))}
        </ul>
      </div>
      <Footer />
    </>
  );
};
