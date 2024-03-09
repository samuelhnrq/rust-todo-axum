const script = document.createElement("script");
script.setAttribute(
  "data-clerk-publishable-key",
  `​pk_test_YmFsYW5jZWQtZWxrLTc2LmNsZXJrLmFjY291bnRzLmRldiQ​`,
);
script.async = true;
script.src = `https://balanced-elk-76.clerk.accounts.dev/npm/@clerk/clerk-js@4/dist/clerk.browser.js`;

script.addEventListener("load", async function () {
  await window.Clerk.load();
  const userButtonComponent = document.querySelector("#user-button");
  window.Clerk.mountUserButton(userButtonComponent);
});

document.body.appendChild(script);

window.addEventListener("load", () => {
  const bootstrap = document.getElementById("bootstrap");
  console.log(bootstrap);
  if (bootstrap.rel === "preload") {
    bootstrap.rel = "stylesheet";
  }
});
