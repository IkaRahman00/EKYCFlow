<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Industry Risk Rating</name>
   <tag></tag>
   <elementGuidId>554fe015-81c8-4f10-a6f9-a8a4ccba1522</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;loanUuid\&quot;: ${ind_loan_id},\n    \&quot;requestUuid\&quot;:${ind_req_id},\n    \&quot;riskAssessmentType\&quot;: \&quot;PROHIBITED_INDUSTRY\&quot;,\n    \&quot;mainApplicantDetails\&quot;: {\n        \&quot;idType\&quot;: \&quot;NRIC\&quot;,\n        \&quot;idValue\&quot;: ${NRIC},\n        \&quot;relations\&quot;: \&quot;OWNER\&quot;,\n        \&quot;stakePercentage\&quot;: 40,\n        \&quot;description\&quot;: \&quot;he is the owner\&quot;\n    },\n    \&quot;businessName\&quot;: \&quot;ABC Pvt Ltd\&quot;,\n    \&quot;otherNames\&quot;: [\n        {\n            \&quot;name\&quot;: \&quot;ABC \u0026 DC Pvt Ltd\&quot;,\n            \&quot;type\&quot;: \&quot;AKA\&quot;,\n            \&quot;description\&quot;: \&quot;Different name\&quot;\n        }\n    ],\n    \&quot;website\&quot;: [\n        \&quot;myboost.com\&quot;\n    ],\n    \&quot;registrationDates\&quot;: [\n        {\n            \&quot;date\&quot;: 1561507200000,\n            \&quot;type\&quot;: \&quot;PRIMARY\&quot;,\n            \&quot;description\&quot;: \&quot;Company was incorporated on this day\&quot;\n        }\n    ],\n    \&quot;registrationPlaces\&quot;: [\n        {\n            \&quot;countryCode\&quot;: \&quot;MY\&quot;,\n            \&quot;region\&quot;: \&quot;Kuala Lumpur\&quot;,\n            \&quot;city\&quot;: \&quot;Kuala Lumpur\&quot;,\n            \&quot;type\&quot;: \&quot;PRIMARY\&quot;,\n            \&quot;description\&quot;: \&quot;Company was incorporated in MY\&quot;\n        }\n    ],\n    \&quot;identificationNumbers\&quot;: [\n        {\n            \&quot;id\&quot;: \&quot;3298223409832\&quot;,\n            \&quot;type\&quot;: \&quot;TAX\&quot;,\n            \&quot;dateOfIssue\&quot;: 1718161920000,\n            \&quot;dateOfExpiry\&quot;: 1718161920000,\n            \&quot;description\&quot;: \&quot;Tax purpose\&quot;\n        }\n    ],\n    \&quot;industry\&quot;: [\n        \&quot;manufacture of paper and paper products\&quot;\n    ],\n    \&quot;activityStatus\&quot;: \&quot;active\&quot;,\n    \&quot;legalForm\&quot;: \&quot;Berhad\&quot;,\n    \&quot;emailAddresses\&quot;: [\n        \&quot;comms@acme.com\&quot;\n    ],\n    \&quot;phoneNumbers\&quot;: [\n        {\n            \&quot;countryCode\&quot;: \&quot;MY\&quot;,\n            \&quot;areaCode\&quot;: \&quot;86\&quot;,\n            \&quot;number\&quot;: \&quot;2983982492\&quot;,\n            \&quot;status\&quot;: \&quot;Active\&quot;,\n            \&quot;type\&quot;: \&quot;LANDLINE\&quot;,\n            \&quot;description\&quot;: \&quot;Current Phone number\&quot;\n        }\n    ],\n    \&quot;registeredAddresses\&quot;: [\n        {\n            \&quot;countryCode\&quot;: \&quot;MY\&quot;,\n            \&quot;region\&quot;: \&quot;string\&quot;,\n            \&quot;city\&quot;: \&quot;string\&quot;,\n            \&quot;areaCode\&quot;: \&quot;string\&quot;,\n            \&quot;zipCode\&quot;: \&quot;string\&quot;,\n            \&quot;streetName\&quot;: \&quot;string\&quot;,\n            \&quot;description\&quot;: \&quot;string\&quot;\n        }\n    ],\n    \&quot;mailingAddresses\&quot;: [\n        {\n            \&quot;countryCode\&quot;: \&quot;MY\&quot;,\n            \&quot;region\&quot;: \&quot;string\&quot;,\n            \&quot;city\&quot;: \&quot;string\&quot;,\n            \&quot;areaCode\&quot;: \&quot;string\&quot;,\n            \&quot;zipCode\&quot;: \&quot;string\&quot;,\n            \&quot;streetName\&quot;: \&quot;string\&quot;,\n            \&quot;description\&quot;: \&quot;string\&quot;\n        }\n    ],\n    \&quot;operatingAddresses\&quot;: [\n        {\n            \&quot;countryCode\&quot;: \&quot;MY\&quot;,\n            \&quot;region\&quot;: \&quot;string\&quot;,\n            \&quot;city\&quot;: \&quot;string\&quot;,\n            \&quot;areaCode\&quot;: \&quot;string\&quot;,\n            \&quot;zipCode\&quot;: \&quot;string\&quot;,\n            \&quot;streetName\&quot;: \&quot;string\&quot;,\n            \&quot;description\&quot;: \&quot;string\&quot;\n        }\n    ],\n    \&quot;businessRegNo\&quot;: \&quot;BusinessRegistrationNumber\&quot;,\n    \&quot;customerRegistrationDate\&quot;: 1718161920000,\n    \&quot;segment\&quot;: \&quot;business\&quot;,\n    \&quot;currencyCode\&quot;: \&quot;MYR\&quot;,\n    \&quot;branches\&quot;: [\n        \&quot;SSSS\&quot;\n    ],\n    \&quot;distributionChannels\&quot;: [\n        \&quot;web\&quot;\n    ],\n    \&quot;creditScore\&quot;: 0,\n    \&quot;totalBalance\&quot;: 1603207359060,\n    \&quot;totalCreditLimit\&quot;: 1603207359060,\n    \&quot;paymentAliases\&quot;: [\n        {\n            \&quot;description\&quot;: \&quot;string\&quot;,\n            \&quot;type\&quot;: \&quot;string\&quot;,\n            \&quot;registeredOn\&quot;: 1718161920000,\n            \&quot;updatedOn\&quot;: 1718161920000,\n            \&quot;status\&quot;: \&quot;string\&quot;\n        }\n    ],\n    \&quot;supplierAddresses\&quot;: [\n        {\n            \&quot;countryCode\&quot;: \&quot;MY\&quot;,\n            \&quot;region\&quot;: \&quot;string\&quot;,\n            \&quot;city\&quot;: \&quot;string\&quot;,\n            \&quot;areaCode\&quot;: \&quot;string\&quot;,\n            \&quot;zipCode\&quot;: \&quot;string\&quot;,\n            \&quot;streetName\&quot;: \&quot;string\&quot;,\n            \&quot;description\&quot;: \&quot;string\&quot;\n        }\n    ],\n    \&quot;riskRating\&quot;: \&quot;string\&quot;,\n    \&quot;income\&quot;: 1603207359060,\n    \&quot;preferredLanguageCode\&quot;: \&quot;ENG\&quot;,\n    \&quot;isActive\&quot;: \&quot;true\&quot;,\n    \&quot;relationshipManager\&quot;: {\n        \&quot;name\&quot;: \&quot;string\&quot;,\n        \&quot;idType\&quot;: \&quot;NRIC\&quot;,\n        \&quot;idValue\&quot;: \&quot;850123146702\&quot;\n    }\n}&quot;,
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
      <webElementGuid>bb8bccd6-7bd2-4b2f-bc33-a898ab5121ae</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>channel</name>
      <type>Main</type>
      <value>WEB</value>
      <webElementGuid>c0a12871-9ee0-43e0-88d0-fe88c5527878</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appId</name>
      <type>Main</type>
      <value>SME_LENDING</value>
      <webElementGuid>bcd8b236-5927-4188-8f0a-013d13eb8297</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appVersion</name>
      <type>Main</type>
      <value>1</value>
      <webElementGuid>f7e88d57-26fd-4857-8627-ff05a6631f80</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>5927dd2e-cc5c-4244-8d0d-a26527dcfe63</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-api.axr-engineering.net/biz-loan-onboarding/v1/framl/checkIndustryRisk</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>8ff1e818-d58c-4815-9746-e9d7050760c4</id>
      <masked>false</masked>
      <name>ind_loan_id</name>
   </variables>
   <variables>
      <defaultValue>9653267</defaultValue>
      <description></description>
      <id>ebc3ed87-c214-4ac6-a416-e221a029fa54</id>
      <masked>false</masked>
      <name>ind_req_id</name>
   </variables>
   <variables>
      <defaultValue>671207035188</defaultValue>
      <description></description>
      <id>3c10ad84-dbc0-4af3-b533-5bc455cb482a</id>
      <masked>false</masked>
      <name>NRIC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>03da2546-79ed-437b-a1cd-b28e33375e36</id>
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
