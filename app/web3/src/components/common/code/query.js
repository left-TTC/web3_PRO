import React, { useState } from "react";
import "../../../utils/aboutquery"
import { checkClass, isCheckDomain, query_domain, WEB3_NAME_SERVICE_ID, WEB3_ROOT,

} from "../../../utils/aboutquery";
import { useNavigate } from "react-router-dom";

const Query = () => {
    const [queryValue, setQueryValue] = useState("");
    const [domainClass, setDomainClass] = useState("");
    const navi = useNavigate();

    const clickToQuery = async() => {
        if(queryValue === ""){
            return; 
        }

        const domainArray = handleQueryDomain(queryValue)
        console.log("domain:", domainArray[0]);
        console.log("class:", domainArray[1]);

        let rootOpt;
        if (isCheckDomain()) {
            rootOpt = WEB3_ROOT;
        }else{
            rootOpt = null;
        }
        const domainClass = checkClass(domainArray[1]);
        const queryResult = await query_domain(
            domainArray[0], WEB3_NAME_SERVICE_ID, domainClass, rootOpt
        );

        navi("/SearchPage", {
            state: {
                queryResult: queryResult,
                queryValue: queryValue,
            }
        })

    };

    return (
        <div className="querybar">
            <input 
                className="leftinput"
                type="text"
                value={queryValue}
                onChange={(e) => setQueryValue(e.target.value)}
                placeholder="start domain search"
            />
            <button className="rightbutton" onClick={clickToQuery}>
                <h1>query</h1>
            </button>
        </div>
    );
}

export default Query;

function handleQueryDomain(input){
    const rawDomain = input;

    if (rawDomain.includes(".")){
        const [part1, part2] = rawDomain.split(".");
        return [part1,part2]
    }else{
        const defaultClass = "web3";
        return [rawDomain, defaultClass]
    }
}

