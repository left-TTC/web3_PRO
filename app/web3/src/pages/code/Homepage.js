import React from "react";
import "../style/Homepage.css";
import "../../components/common/style/query.css"
import Query from "../../components/common/code/query"

function HomePage() {
  return (
    <div className="home-page">
      <div className="spacefornav"></div>
      <div className="introduceandserachbar">
        <div className="introduce">
          <h1>web3 domain in anywhere</h1>
        </div>
        <Query/>
      </div>
    </div>
  );
}

export default HomePage;