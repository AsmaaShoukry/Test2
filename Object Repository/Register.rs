<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Register</name>
   <tag></tag>
   <elementGuidId>5ad1aaa7-f13a-44ae-a899-3a6d2e89537e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;name&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Ahmed Abdeen&quot;
    },
    {
      &quot;name&quot;: &quot;email&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Abdeen.Ahmed91@gmail.com&quot;
    },
    {
      &quot;name&quot;: &quot;phone_number&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;01060603234&quot;
    },
    {
      &quot;name&quot;: &quot;type&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Buyer&quot;
    },
    {
      &quot;name&quot;: &quot;preferred_language&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;en&quot;
    },
    {
      &quot;name&quot;: &quot;password&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;ASD@123a&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://shikaty-core.dev.brightcreations.com/api/v1/register?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
