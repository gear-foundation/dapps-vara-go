import varaMainLogo from "@/assets/icons/logo-vara-go.png";
import advLogo from "@/assets/icons/adv.png";
import styles from "./Footer.module.scss";

export const Footer = () => {
  return (
    <footer className={styles.footer}>
      <div className={styles.logo}>
        <img
          src={varaMainLogo}
          alt="vara main logo"
          className={styles.logoImage}
        />
      </div>
      <img src={advLogo} alt="advertisment" className={styles.logoImage} />
      <div className={styles.footerRights}>Copyright (C) 2023 Vara GO</div>
    </footer>
  );
};
