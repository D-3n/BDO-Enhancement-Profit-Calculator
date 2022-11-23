# BDO-Enhancement-Profit-Calculator
Calculates profits from enhancing in Black Desert.

### Compilation Instructions
When compiling, three environment variables need to be set: 
<ol>
    <li> <strong>BDO_COOKIE_TRADE_AUTH</strong>
    <li> <strong>BDO_COOKIE_REQUEST_VERIFICATION_TOKEN</strong>
    <li> <strong>BDO_QUERY_REQUEST_VERFICATION_TOKEN</strong>
</ol>
These values need to be set from your central market.
Instructions are provided in <code>/src/bdo_market_requests/bdo_post_requests.rs</code>

### To do

In no particular order:

- [x] Add support for other regions
- [] Allow custom failstack input
- [] Add calculation of failstack costs
- [] Multithread more complex operations
- [] Improve error handling
- [] Add functionality for different levels of enhancements (for accessories)
- [] Create simple UI (maybe)
- [] Improve handling of POST requests
- [] 