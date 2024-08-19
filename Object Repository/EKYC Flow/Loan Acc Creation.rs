<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Loan Acc Creation</name>
   <tag></tag>
   <elementGuidId>cd4d14bf-7504-4041-a769-bad211be7098</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;loanUuid\&quot;: ${crloan_id},\n    \&quot;requestUuid\&quot;:${crreq_id},\n    \&quot;loanType\&quot;: \&quot;NEW\&quot;,\n    \&quot;loanProductId\&quot;: \&quot;TERM_LOAN\&quot;,\n    \&quot;loanAmount\&quot;: 2300.00,\n    \&quot;currencyCode\&quot;: \&quot;MYR\&quot;,\n    \&quot;businessRegNumber\&quot;:${BRN}, \n    \&quot;interestRate\&quot;: 15.00, \n    \&quot;guarantors\&quot;: [ \n        {\n            \&quot;idType\&quot;: \&quot;NRIC\&quot;,\n            \&quot;idValue\&quot;: ${NRIC},\n            \&quot;guarantorAmount\&quot;: 7500.00,\n            \&quot;currencyCode\&quot;: \&quot;MYR\&quot;\n        }\n    ],\n    \&quot;tenure\&quot;:{  \n        \&quot;unitCount\&quot;:10,\n        \&quot;unit\&quot;:\&quot;MONTHS\&quot;\n    },    \n    \&quot;disbursementSettings\&quot;: {   \n        \&quot;creditorName\&quot;: \&quot;John Doe\&quot;, \n        \&quot;creditorAccNo\&quot;: \&quot;123456\&quot;,\n        \&quot;creditorBankName\&quot;: \&quot;ABC Bank\&quot;,\n        \&quot;creditorBankId\&quot;:\&quot;\&quot;,\n        \&quot;creditorBranchName\&quot;: \&quot;\&quot;,\n        \&quot;disbursementAmount\&quot;:\&quot;0.00\&quot;,\n        \&quot;fees\&quot;: [  \n            {\n                \&quot;type\&quot;: \&quot;Tax\&quot;,\n                \&quot;amount\&quot;: 0.00,            \n                \&quot;percentage\&quot;: 0.00\n            }\n        ]\n    },\n    \&quot;repaymentSettings\&quot;: {   \n        \&quot;consentId\&quot;: \&quot;68476585654867549675\&quot;, \n        \&quot;consentAssociatedAmount\&quot;:0.00,\n        \&quot;consentGivenDateTime\&quot;: \&quot;2016-09-06T13:37:50+03:00\&quot;,\n        \&quot;collectionAccHolderName\&quot;:\&quot;Jane Doe\&quot;,\n        \&quot;collectionAccNo\&quot;:\&quot;123456\&quot;,\n        \&quot;collectionBankName\&quot;:\&quot;XYZ Bank\&quot;,\n        \&quot;collectionBankId\&quot;:\&quot;\&quot;,\n        \&quot;collectionBankBranchName\&quot;:\&quot;\&quot;,\n        \&quot;fees\&quot;: [ \n            {\n                \&quot;type\&quot;: \&quot;Tax\&quot;,\n                \&quot;amount\&quot;: 0.00,\n                \&quot;percentage\&quot;: 0.00\n            }\n        ]\n    }\n}&quot;,
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
      <webElementGuid>88cab217-a1f1-4b74-92a4-50f136e8317d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>channel</name>
      <type>Main</type>
      <value>WEB</value>
      <webElementGuid>94443aff-3483-4a48-a35c-ac3a6b682176</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accept</name>
      <type>Main</type>
      <value>*/*</value>
      <webElementGuid>6a881f5a-7e37-431e-af6d-e5d70269026e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appId</name>
      <type>Main</type>
      <value>SME_LENDING</value>
      <webElementGuid>076245ed-ffa5-4090-a811-d52aee2eba46</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appVersion</name>
      <type>Main</type>
      <value>V6.0</value>
      <webElementGuid>7b44fed8-d0ff-425c-8e40-369d6d5edefc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>79708253-ef70-4d66-8a06-04c1a9f2bfdb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-api.axr-engineering.net/biz-loan-onboarding/v1/loan</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>99f9c831-83d0-4d16-8e55-5a78812aa5c3</id>
      <masked>false</masked>
      <name>Token</name>
   </variables>
   <variables>
      <defaultValue>3992866831</defaultValue>
      <description></description>
      <id>e87d9da1-3fa3-4d6e-950f-58867484c8b7</id>
      <masked>false</masked>
      <name>crloan_id</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>2e2490fc-b7c2-4d3f-b634-0bf29e0a21f5</id>
      <masked>false</masked>
      <name>crreq_id</name>
   </variables>
   <variables>
      <defaultValue>4171572069</defaultValue>
      <description></description>
      <id>0c9e598e-d2b3-4eae-b4eb-65ec8c153802</id>
      <masked>false</masked>
      <name>BRN</name>
   </variables>
   <variables>
      <defaultValue>1</defaultValue>
      <description></description>
      <id>a69300e8-7198-4c26-9675-2ac19a87f5a2</id>
      <masked>false</masked>
      <name>NRIC</name>
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
