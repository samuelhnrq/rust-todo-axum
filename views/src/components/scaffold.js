"use strict";

function copyPreloads() {
  if (document.readyState !== "interactive") return;
  const preloads = document.querySelectorAll('link[rel="preload"]');
  preloads.forEach((preloadLink) => {
    const realLink = preloadLink.cloneNode();
    if (realLink.href.endsWith(".css")) {
      realLink.rel = "stylesheet";
    }
    preloadLink.after(realLink);
  });
  document.removeEventListener("readystatechange", copyPreloads);
}

document.addEventListener("readystatechange", copyPreloads);
copyPreloads();
