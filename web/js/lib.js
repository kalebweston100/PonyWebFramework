let app = null;

const load_page = (pagename) => {
    let current_page = document.querySelector("#" + pagename);
    if (app) {
        app.innerHTML = current_page.innerHTML;
    }
}

window.onload = () => {

    app = document.querySelector("#app");

    for (let i = 0; i < pages.length; i++) {
        let temp_page = document.querySelector("#" + pages[i]);
        temp_page.style.visibility = "hidden";
    }

    load_page(pages[0]);
}

