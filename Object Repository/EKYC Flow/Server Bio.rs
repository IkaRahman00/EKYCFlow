<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Server Bio</name>
   <tag></tag>
   <elementGuidId>da0dfe6d-5670-422f-8314-6ead41d2ebd9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n  \&quot;idType\&quot;: \&quot;NRIC\&quot;,\n  \&quot;idValue\&quot;: ${NRIC},\n  \&quot;loanUuid\&quot;: ${ser_loan_id},\n  \&quot;requestUuid\&quot;: ${ser_req_id},\n  \&quot;redirectUrl\&quot;: \&quot;https://test.axr-engineering.net/\&quot;\n}&quot;,
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
      <webElementGuid>dee8f3f6-c1e2-4d91-b0e1-514d8df0c940</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accept</name>
      <type>Main</type>
      <value>*/*</value>
      <webElementGuid>4a03bd2a-6a49-4bd6-9ccf-a8ea65a37501</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>channel</name>
      <type>Main</type>
      <value>WEB</value>
      <webElementGuid>34fb52a8-2662-4b22-a61c-1ecb27c31684</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appId</name>
      <type>Main</type>
      <value>SME_LENDING</value>
      <webElementGuid>30c5988b-8244-4958-822f-0729ef31e6df</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token}</value>
      <webElementGuid>df86bb87-b0a7-4210-ab64-34e4fa341b63</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://test-api.axr-engineering.net/biz-loan-onboarding/v1/ekyc/serverBio</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>1233244</defaultValue>
      <description></description>
      <id>e6f85f03-675b-4692-b9e0-0b30709074b0</id>
      <masked>false</masked>
      <name>ser_loan_id</name>
   </variables>
   <variables>
      <defaultValue>23123343</defaultValue>
      <description></description>
      <id>60dd6a77-63db-42e5-8c5d-9604f00762c4</id>
      <masked>false</masked>
      <name>ser_req_id</name>
   </variables>
   <variables>
      <defaultValue>199685304173</defaultValue>
      <description></description>
      <id>9cb987d8-8ece-45db-a296-a534e2ae905d</id>
      <masked>false</masked>
      <name>NRIC</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Token</defaultValue>
      <description></description>
      <id>01911712-d6be-45e1-b9cf-b6ed43d8e7bb</id>
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
