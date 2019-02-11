<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Login</name>
   <tag></tag>
   <elementGuidId>85c1a50d-3cf0-46c7-a191-95ccb06492be</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;:\&quot;${EMAIL}\&quot;,\n  \&quot;password\&quot;:\&quot;${PASSWORD}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>device</name>
      <type>Main</type>
      <value>web</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>jwt</name>
      <type>Main</type>
      <value>true</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>businessTagId</name>
      <type>Main</type>
      <value>${BUSINESS_TAG_ID}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://${BASE_URL}/${API_VERSION}/user/login?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>ab33ab8f-85e8-450a-b451-c37f8a4c1937</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.API_VERSION</defaultValue>
      <description></description>
      <id>319d52a7-8f67-4d03-96b6-c2b7bc93e77e</id>
      <masked>false</masked>
      <name>API_VERSION</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.EMAIL</defaultValue>
      <description></description>
      <id>20c5b9be-8e19-44c1-805c-400fcf224bf1</id>
      <masked>false</masked>
      <name>EMAIL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.PASSWORD</defaultValue>
      <description></description>
      <id>332a1f73-3c74-41b5-9a4b-9ca8dca42e99</id>
      <masked>false</masked>
      <name>PASSWORD</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BUSINESS_TAG_ID</defaultValue>
      <description></description>
      <id>9f5f1f82-aefb-48b0-9e7d-920c453c4b1f</id>
      <masked>false</masked>
      <name>BUSINESS_TAG_ID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'error', 'false')

WS.containsString(response, 'loginToken', false)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
