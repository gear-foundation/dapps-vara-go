export const styles = `
<style>
      .app__content {
        height: 100%;
        display: flex;
        flex: 1;
        padding: 12px 50px;
      }

      .app__container {
        height: 100%;
        display: flex;
        flex-direction: column;
      }

      .app__content__left {
        background: #fff;
        width: 60%;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        flex: 1;
      }

      .app__content__left__header {
        height: 60px;
        padding-right: 60px;
      }

      .app__content__left__header__menu {
        display: flex;
        justify-content: end;
        align-items: center;
        height: 100%;
        gap: 40px;
      }

      .app__content__left__header__menu__item {
        text-transform: capitalize;
        font-size: 16px;
        font-weight: 600;
        cursor: pointer;
      }

      .app__content__left__content {
        width: 70%;
        padding: 20px 20px 100px 80px;
        box-sizing: border-box;
      }

      .app__content__right__header {
        height: 60px;
        padding-left: 60px;
      }

      .socials {
        height: 100%;
        width: 100%;
        display: flex;
        align-items: center;
        gap: 26px;
      }

      .socials__item {
        fill: #ffffff;
        cursor: pointer
      }

      .content__heading {
        font-size: 58px;
        font-weight: 700;
        padding-bottom: 50px;
      }

      .content__text {
        font-size: 20px;
        font-weight: 500;
      }

      .app__content__right {
        position: relative;
        background: #5b2b70;
        width: 40%;
        height: 100%;
      }

      .app__content__right__picture {
        position: absolute;
        width: 700px;
        right: 30px;
        bottom: 0;
      }
      </style>`;

export const styles1 = `
  <style>
  .header {
    background: #333;
    color: #fff;
    padding: 20px;
  }
  
  .header-title {
    margin-bottom: 0;
  }
  

  .main-content {
    width: 80%;
    margin: 0 auto;
  }
  

  .post {
    margin-top: 60px;
    margin-bottom: 40px;
  }
  
  .post-title {
    border-bottom: 1px solid #ddd;
    padding-bottom: 10px;
  }
  
  .post-intro,
  .post-description,  
  .post-conclusion {
    line-height: 1.5;
  }
  
  .post-details {
    margin-bottom: 10px;
  }
  

  .post-list {
    padding-left: 20px;
  }
  </style>
`;

export const styles2 = `
  <style>
  .header {
    background: #333;
    color: #fff;
    padding: 20px;
  }
  
  .header-title {
    margin-bottom: 0;
  }
  

  .main-content {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 48px;
    font-weight: 600;
  }

  </style>
`;
