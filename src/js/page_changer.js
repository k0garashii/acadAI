const contentElement = document.getElementById("content");
const navLinks = document.querySelectorAll("nav ul li a");

function changeScene(pageId) {
  // Masquer toutes les pages
  const allPages = document.querySelectorAll("main.container");
  allPages.forEach(page => {
    page.style.display = "none";
  });

  // Afficher la page demandée
  const selectedPage = document.getElementById(pageId);
  selectedPage.style.display = "block";

  // Mettre à jour l'historique
  window.history.pushState({ page: pageId }, "", pageId);
}

// Gérer les clics sur les liens de navigation
navLinks.forEach(link => {
  link.addEventListener("click", (event) => {
    event.preventDefault();
    const pageId = link.dataset.page;
    changeScene(pageId);
  });
});

window.addEventListener("popstate", (e) => {
  const pageId = e.state ? e.state.page : "page0"; // Page 0 par défaut
  changeScene(pageId);
});

// Charger la première page au chargement initial
changeScene("page0");