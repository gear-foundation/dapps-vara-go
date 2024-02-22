import { useState } from "react";
import { motion } from "framer-motion";
import { ReactComponent as DonateSVG } from "@/assets/icons/donate.svg";
import { ReactComponent as BurgerMenuSVG } from "@/assets/icons/burger-menu.svg";
import { ReactComponent as CrossIconSVG } from "@/assets/icons/cross-icon.svg";
import varaMainLogo from "@/assets/icons/logo-vara-go.png";
import styles from "./Header.module.scss";
import { Wallet } from "@/features/wallet-new";
import { SearchInput } from "../SearchInput";
import { Link } from "react-router-dom";

export const Header = () => {
  const [isMenuOpen, setIsMenuOpen] = useState<boolean>(false);

  const handleOpenMenu = () => {
    setIsMenuOpen(true);
  };

  const handleCloseMenu = () => {
    setIsMenuOpen(false);
  };

  return (
    <header className={styles.header}>
      <Link to={"/"}>
        <button className={styles.logo}>
          <img
            src={varaMainLogo}
            alt="vara main logo"
            className={styles.logoImage}
          />
        </button>
      </Link>
      <SearchInput />
      <div className={styles.headerRight}>
        <button className={styles.donateButton}>
          <DonateSVG /> DONATE
        </button>
        <Wallet />
        <button className={styles.rightButton}>
          {isMenuOpen ? (
            <CrossIconSVG
              className={styles.headerButton}
              onClick={handleCloseMenu}
            />
          ) : (
            <BurgerMenuSVG
              className={styles.headerButton}
              onClick={handleOpenMenu}
            />
          )}
        </button>
      </div>
      {isMenuOpen && (
        <motion.div
          className={styles.subheaderWrapper}
          initial={{ opacity: 0 }}
          whileInView={{ opacity: 1 }}
        >
          <div className={styles.subheader}>
            <ul className={styles.subheaderMenu}>
              <li className={styles.subheaderMenuItem}>Subscribe</li>
              <li className={styles.subheaderMenuItem}>Store</li>
              <li className={styles.subheaderMenuItem}>Contact</li>
              <li className={styles.subheaderMenuItem}>Advertise</li>
              <li className={styles.subheaderMenuItem}>Cancel subscription</li>
            </ul>
          </div>
        </motion.div>
      )}
    </header>
  );
};
