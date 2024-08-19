<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Test_Customer Risk Rating</name>
   <tag></tag>
   <elementGuidId>e73633f9-0a33-43cf-a60a-9c866b1f3f31</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;loanUuid\&quot;: ${cust_loan_id},\n    \&quot;requestUuid\&quot;: ${cust_req_id},\n    \&quot;customerIdType\&quot;: \&quot;NRIC\&quot;,\n    \&quot;customerIdValue\&quot;: ${NRIC},\n    \&quot;customerName\&quot;: \&quot;Trevor Pitkowski\&quot;,\n    \&quot;customerEmail\&quot;: \&quot;johndoe@example.com\&quot;,\n    \&quot;customerPhone\&quot;: \&quot;+1234567890\&quot;,\n    \&quot;customerPhoneValCode\&quot;: \&quot;001\&quot;,\n    \&quot;customerCitizenIdNumber\&quot;: \&quot;11111\&quot;,\n    \&quot;customerPassportNumber\&quot;: \&quot;P1234567\&quot;,\n    \&quot;customerDriverLicense\&quot;: \&quot;D1234567\&quot;,\n    \&quot;customerSsn\&quot;: \&quot;123-45-6789\&quot;,\n    \&quot;customerTaxNumber\&quot;: \&quot;T1234567\&quot;,\n    \&quot;customerStreetAddr\&quot;: \&quot;123 Main\&quot;,\n    \&quot;customerZip\&quot;: \&quot;12345\&quot;,\n    \&quot;customerZipAreaCode\&quot;: \&quot;123\&quot;,\n    \&quot;customerCity\&quot;: \&quot;Anytown\&quot;,\n    \&quot;customerRegion\&quot;: \&quot;Anystate\&quot;,\n    \&quot;customerCountryCode\&quot;: \&quot;US\&quot;,\n    \&quot;customerDob\&quot;: \&quot;1980-01-01\&quot;,\n    \&quot;customerNumberOfAccounts\&quot;: 5,\n    \&quot;customerTotalBalance\&quot;: 1000000,\n    \&quot;customerTotalCreditLimit\&quot;: 500000,\n    \&quot;customerKycLvl\&quot;: \&quot;HIGH\&quot;,\n    \&quot;customerOccupation\&quot;: \&quot;Engineer\&quot;,\n    \&quot;customerSegment\&quot;: \&quot;VIP\&quot;,\n    \&quot;customerNumberOfCards\&quot;: 3,\n    \&quot;customerOccupationStatus\&quot;: \&quot;Employed\&quot;,\n    \&quot;customerRiskRating\&quot;: \&quot;LOW\&quot;,\n    \&quot;customerCurrency\&quot;: \&quot;USD\&quot;,\n    \&quot;customerAlias\&quot;: \&quot;JDoe\&quot;,\n    \&quot;customerAliasType\&quot;: \&quot;Primary\&quot;,\n    \&quot;customerAliasRegistrationTime\&quot;: 1609459200,\n    \&quot;customerAliasLastUpdateTime\&quot;: 1612137600,\n    \&quot;customerAliasStatus\&quot;: \&quot;active\&quot;,\n    \&quot;customerReportedIncome\&quot;: 100000,\n    \&quot;customerPreferredLanguage\&quot;: \&quot;EN\&quot;,\n    \&quot;customerBranchId\&quot;: \&quot;BR123\&quot;,\n    \&quot;customerCitizenship\&quot;: \&quot;US\&quot;,\n    \&quot;customerMailingCountryCode\&quot;: \&quot;MY\&quot;,\n    \&quot;customerMailingStreetAddress\&quot;: \&quot;123 Main\&quot;,\n    \&quot;customerMailingZip\&quot;: \&quot;12345\&quot;,\n    \&quot;customerBusinessActivityCountries\&quot;: [\n        \&quot;US\&quot;,\n        \&quot;CA\&quot;\n    ],\n    \&quot;customerBusinessActivityType\&quot;: \&quot;Manufacturing\&quot;,\n    \&quot;customerBusinessIdNumber\&quot;: \&quot;BID12345\&quot;,\n    \&quot;customerBusinessIncorporationCountry\&quot;: \&quot;US\&quot;,\n    \&quot;customerBusinessLegalEntityType\&quot;: \&quot;LLC\&quot;,\n    \&quot;customerBusinessSuppliersCountries\&quot;: [\n        \&quot;US\&quot;,\n        \&quot;CA\&quot;,\n        \&quot;MX\&quot;\n    ],\n    \&quot;screeningTags\&quot;: [\n        \&quot;SAN\&quot;,\n        \&quot;PEP\&quot;\n    ],\n    \&quot;customerActivityType\&quot;: \&quot;Manufacturing\&quot;,\n    \&quot;customerDistributionChannels\&quot;: [\n        \&quot;MOBILE\&quot;\n    ],\n    \&quot;customerHighNetworth\&quot;: true,\n    \&quot;customerType\&quot;: \&quot;BUSINESS\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>eaef2edd-951a-426a-8bc1-edabf7068f36</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>channel</name>
      <type>Main</type>
      <value>WEB</value>
      <webElementGuid>06ab5a22-ecb5-435b-830c-b9d8d43f7501</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appId</name>
      <type>Main</type>
      <value>SME_LENDING</value>
      <webElementGuid>8d5c5664-5c4c-46ff-9c73-5cb66ed31bf4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appVersion</name>
      <type>Main</type>
      <value>v1.0</value>
      <webElementGuid>da4d6e19-ff53-4e9d-8d8a-c9e7affd4a92</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>7e8b4efe-39ed-47ec-ac40-77eaf8047466</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-api.axr-engineering.net/biz-loan-onboarding/v1/framl/customerRiskRating</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>973523283373322</defaultValue>
      <description></description>
      <id>6184e7d2-c118-47c4-a965-88260e167acb</id>
      <masked>false</masked>
      <name>cust_req_id</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>75edbfb6-a191-4775-90bf-4229fc2eab18</id>
      <masked>false</masked>
      <name>cust_loan_id</name>
   </variables>
   <variables>
      <defaultValue>671207035185</defaultValue>
      <description></description>
      <id>b212cb13-7802-4d8b-a60a-08628260a794</id>
      <masked>false</masked>
      <name>NRIC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>6d619cf7-21a5-40d2-acd9-6088e23a6b43</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
