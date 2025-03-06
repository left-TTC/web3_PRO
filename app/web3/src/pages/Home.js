// src/pages/Home.js
import React, {useState} from "react";
import { Link, useNavigate } from "react-router-dom";
import './Home.css';
import {check_domain_availability}  from '../components/contract_function';  

export default function Home() {
  const [domain, setDomain] = useState('');
  //get navigate hook
  const navigate = useNavigate();
  //set value to save buyerwallet
  const [buyer_wallet, setWallet] = useState('');

  const handleCheckDomain = async () => {
    try {
      const result = await check_domain_availability(domain); 
      if(result !== ""){
        navigate('./about', {state: {pda: result, domain: domain}})
      }
    } catch (error) {
      console.log("error:", error)
    }
  };


  //return HTML space
  return (
    <div className="home-container">
       <input 
        type="text"
        value={domain} 
        onChange={(e) => setDomain(e.target.value)} 
        placeholder="input"
        className="domain-input" 
      />
      <button onClick={handleCheckDomain}>Check Domain</button>
    </div>
  );
}

