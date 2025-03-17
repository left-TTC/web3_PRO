import React, { useState } from "react";

function SearchPage() {
  const [domain, setDomain] = useState("");

  const handleSearch = () => {
    console.log("Searching for domain:", domain);
    // 这里可以添加搜索逻辑
  };

  return (
    <div className="search-page">
      <h2>Search for a .web3 Domain</h2>
      <input
        type="text"
        value={domain}
        onChange={(e) => setDomain(e.target.value)}
        placeholder="Enter a domain name"
      />
      <button onClick={handleSearch}>Search</button>
    </div>
  );
}

export default SearchPage;