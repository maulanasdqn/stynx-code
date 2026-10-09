import { mount } from "svelte";
import App from "./app.svelte";
import { installEasings } from "./lib/motion.js";
import "./app.css";
import "./glass.css";

installEasings();

export default mount(App, { target: document.getElementById("app") });
