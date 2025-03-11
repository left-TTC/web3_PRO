import React from 'react';



const TopBar = () => {

    const handleClick = (item) => {
        console.log(`${item} clicked`);
      };

    return (
        <div style={styles.header}>
          <div style={styles.section}>
            {['SAR', 'Browser domain', 'Info', 'Contact', 'Search'].map((item) => (
              <div key={item} style={styles.item} onClick={() => handleClick(item)}>
                {item}
              </div>
            ))}
          </div>
          <div style={styles.section}>
            <div style={styles.item} onClick={() => handleClick('Connect wallet')}>
              Connect wallet
            </div>
          </div>
        </div>
      );
    };
    
    const styles = {
      header: {
        display: 'flex',
        justifyContent: 'space-between',
        padding: '10px',
        backgroundColor: '#f4f4f4',
        borderBottom: '1px solid #ccc',
      },
      section: {
        display: 'flex',
        gap: '20px',
      },
      item: {
        cursor: 'pointer',
      },
};

export default TopBar;