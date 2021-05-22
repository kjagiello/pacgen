const rules = {};

function isAllowed(rule, host) {
  // If no allowedHosts provided, accept the rule.
  if (rule.allowedHosts === null) {
    return true;
  }

  // Check if the host matches any of the allowed hosts.
  if (rule.allowedHosts.some(allowedHost => shExpMatch(host, allowedHost))) {
    return true;
  }

  return false;
}

function FindProxyForURL(url, host) {
  const matchingRule = rules.find(rule => isAllowed(rule, host));

  // If a matching rule found, return the proxy settings for it.
  if (matchingRule) {
    return matchingRule.proxies;
  }

  // If none of the rules have matched, don't use any proxy.
  return "DIRECT";
}
