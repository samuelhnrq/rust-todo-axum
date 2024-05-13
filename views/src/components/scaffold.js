"use strict";

function copyPreloads() {
  const preloads = document.querySelectorAll('link[rel="preload"]');
  console.log("found", preloads.length, "links");
  preloads.forEach((x) => {
    const realLink = x.cloneNode();
    if (realLink.href.endsWith(".css")) {
      realLink.rel = "stylesheet";
    }
    x.after(realLink);
  });
}

window.addEventListener("load", copyPreloads, { once: true });
if (document.readyState == "interactive") {
  copyPreloads();
}
