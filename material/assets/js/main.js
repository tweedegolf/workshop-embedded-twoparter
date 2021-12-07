/* Ugly but working script that makes presenters work with mdbook */
document.addEventListener('DOMContentLoaded', () => {
    const hrefNext = document.querySelector('.nav-chapters.next');
    const hrefPrev = document.querySelector('.nav-chapters.previous');

    document.addEventListener('keydown', (e) => {
        if (e.key == "PageDown") {
            hrefNext && hrefNext && (window.location.href = hrefNext.href);
            e.preventDefault();
            return true;
        }
        if (e.key == "PageUp") {
            hrefNext && hrefPrev && (window.location.href = hrefPrev.href);
            e.preventDefault();
            return true;
        }
    });
});