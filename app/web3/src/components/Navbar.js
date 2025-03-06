import React from "react";
import { Link } from "react-router-dom";

export default function Navbar() {
  return (
    <nav style={{ padding: "10px", background: "#333", color: "#fff" }}>
      <Link to="/" style={{ margin: "10px", color: "white" }}>Home</Link>
      <Link to="/about" style={{ margin: "10px", color: "white" }}>About</Link>
      <Link to="/contact" style={{ margin: "10px", color: "white" }}>Contact</Link>
    </nav>
  );
}