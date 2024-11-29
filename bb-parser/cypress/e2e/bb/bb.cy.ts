import * as secret from "../../secret";

const visitLanding = () => {
  cy.visit(secret.landing, { headers: { cookie: secret.cookie } });
  cy.wait(5000);
//   cy.get('#j_username').type(secret.login);
//   cy.get('#j_password').type(secret.password);
//   cy.get('#submit').click();
//   cy.wait(10000);
};

const scrollUntilPosssible = async () => {
  cy.scrollTo("bottom");
  cy.wait(3000).then(() => {
    scrollUntilPosssible();
  });
};

const searchForComment = (url) => {
  cy.visit(url);
  cy.wait(3000);
  scrollUntilPosssible();
};

describe("bb", () => {
  it("should visit", () => {
    visitLanding();
    secret.prs.forEach(searchForComment);
  });
});
