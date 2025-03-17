import React from "react";
import Navbar from "../components/global/Navbar";
import Footer from "../components/global/Footer";

function MainLayout({ children }) {
  return (
    <div className="main-layout">
      <Navbar />
      <main>{children}</main>
      <Footer />
    </div>
  );
}

export default MainLayout;