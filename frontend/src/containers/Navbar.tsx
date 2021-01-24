import React from "react";
import "../static/Navbar.css";
import { Link } from "react-router-dom";

export default function Navbar() {
  return (
    <nav className="navbar">
      <Link to="/" className="navElement">
        Home
      </Link>
      <Link to="/projects" className="navElement">
        Projects
      </Link>
      <Link to="/contact-me" className="navElement">
        Contact me
      </Link>
    </nav>
  );
}
