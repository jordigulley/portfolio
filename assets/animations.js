import { stagger, animate, spring, scroll } from "https://cdn.jsdelivr.net/npm/motion@latest/+esm"
function animate_name() {
    // Only play animation if we are not coming from a link in this site.
    if (document.referrer.includes(location.hostname)) {
        return;
    }
    const hey_im = document.querySelector("#hey_im");
    const first_name = document.querySelector("#first_name_animate");
    const last_name = document.querySelector("#last_name_animate");
    const navbar = document.querySelector("#navbar");
    const content = document.querySelector("#content");
    const portrait = document.querySelector(".portrait");
    // Set initial animation state
    hey_im.style.transform = "translateY(15.0px)";
    hey_im.style.opacity = 0.0;
    const first_and_last_name = [first_name, last_name];
    first_and_last_name.forEach((el) => {
        el.style.transform = "scale(0.0) rotate(-90deg)";
        el.style.opacity = 0.0;
    });
    navbar.style.transform = "translateY(15.0px)";
    navbar.style.opacity = 0.0;
    content.style.opacity = 0.0;
    // portrait.style.transform = "scale(0.0)";
    portrait.style.opacity = 0.0;
    // Setup animations
    const hey_im_animation_data = [
        hey_im,
        {
            translateY: 0.0,
            opacity: 1.0,
        },
        { duration: 0.25 }
    ];
    const first_and_last_name_animation_data = [
        first_and_last_name,
        {
            scale: 1.0,
            rotate: 0.0,
            opacity: 1.0,
        },
        { duration: 0.5, delay: stagger(0.35), type: spring, stiffness: 300,  at: "0.5"}
    ];
    const navbar_animation_data = [
        navbar,
        {
            translateY: 0.0,
            opacity: 1.0,
        },
        { duration: 0.5, type: spring, stiffness: 300, at: "-0.8" }
    ];
    const content_animation_data = [
        content,
        {
            opacity: 1.0,
        },
        { duration: 0.5, at: "<" }
    ];
    const portrait_animation_data = [
        portrait,
        {
            opacity: 1.0,
            scale: 1.0,
        },
        { duration: 0.5, type: spring }
    ];
    // Execute animation sequence
    animate([
        hey_im_animation_data,
        first_and_last_name_animation_data,
        navbar_animation_data,
        content_animation_data,
        portrait_animation_data
    ]);
}
window.addEventListener("DOMContentLoaded", () => animate_name());