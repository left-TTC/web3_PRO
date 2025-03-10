import React from "react";
import { useLocation } from "react-router-dom";
import './About.css'

export default function About() {
  const location = useLocation();
  const { pda, domain } = location.state || {};
  
  // async function confirm_to_buy() {
  //   call_web3_register(domain,pda);
  // }

  return (
    <div className="buy_container">
      
    </div>
  );
}