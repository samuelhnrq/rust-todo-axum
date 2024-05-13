async function initAuth() {
  let signIn = document.getElementById("sign-in");
  let container = document.getElementById("clerk-container");
  await Clerk.load();
  signIn.addEventListener("click", () => {
    Clerk.openSignIn();
  });
  if (Clerk.session?.status === "active") {
    let user = document.createElement("button");
    container.appendChild(user);
    Clerk.mountUserButton(user, { afterSignOutUrl: window.location.origin });
  } else {
    signIn.style.display = null;
  }
}

document
  .getElementById("auth-script-tag")
  .addEventListener("load", initAuth, { once: true });
