import { Footer } from "@/components/Footer";
import styles from "./SearchResults.module.scss";
import { Link } from "react-router-dom";

const results = [
  {
    name: "The Searching algorithms are used to search or find one or more than one element from a dataset",
    url: "url1",
  },
  {
    name: "The search engines Internet are sites designed to provide information on what you are",
    url: "url2",
  },
  {
    name: "Some of the features that make Wolfram Alpha stand out from other search engines include ",
    url: "url3",
  },
  {
    name: "A search engine is a web based tool that is used by people to locate information on the internet. Some of the most popular examples of search engines are Google",
    url: "url4",
  },
  {
    name: "Below are some tried and true web search techniques that work with virtually any search",
    url: "url5",
  },
  {
    name: "It is a good idea to learn more about some general search principles, such as combining search terms or using synonyms and truncation,",
    url: "url6",
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
